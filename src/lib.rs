#![feature(path_try_exists)]

use colored::Colorize;
use dialoguer::theme::SimpleTheme;
use dialoguer::{Confirm, Input, MultiSelect};
use fs_extra::dir::get_size;
use home::home_dir;
use std::fs::remove_dir_all;
use std::path::Path;
use walkdir::WalkDir;

/// Returns the User directory, or the filesystem root if no User directory is found
pub fn get_home_dir() -> String {
    let hd = home_dir();
    let home_directory = match hd.as_ref() {
        None => "",
        Some(dir) => dir.to_str().unwrap(),
    };

    String::from(home_directory)
}

pub fn get_node_module_paths(starting_directory: String) -> Vec<(String, u64)> {
    Path::new(&starting_directory)
        .try_exists()
        .expect("Path doesn't exist");

    let mut node_modules: Vec<(String, u64)> = WalkDir::new(starting_directory)
        .follow_links(true)
        .into_iter()
        .filter(|e| e.is_ok())
        .filter(|e| {
            let matches = e
                .as_ref()
                .unwrap()
                .path()
                .to_string_lossy()
                .matches("node_modules")
                .count();
            return matches == 1
                && e.as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .ends_with("node_modules");
        })
        .filter(|e| e.as_ref().unwrap().path().exists())
        .map(|e| {
            let owned_e = e.unwrap();
            return (
                owned_e.path().to_str().unwrap().to_owned(),
                get_size(owned_e.path()).unwrap_or(0),
            );
        })
        .collect();

    node_modules.sort_unstable_by_key(|k| k.1);
    node_modules.reverse();
    node_modules
}

pub fn main() {
    let home_dir = get_home_dir();

    let starting_directory: String = Input::new()
        .with_prompt("Where should I start searching?")
        .default(home_dir)
        .interact_text()
        .unwrap();

    let node_modules = get_node_module_paths(starting_directory);

    let _count = node_modules.len();
    let _total_size =
        human_bytes::human_bytes(node_modules.iter().fold(0, |acc, x| acc + x.1) as f64);

    println!("Found {_count} node_module folders with a total size of {_total_size}");

    let paths: Vec<String> = node_modules.iter().map(|e| e.0.clone()).collect();

    let names: Vec<String> = node_modules
        .iter()
        .map(|e| {
            let color = match e.1 {
                1..=100_000_000 => "green",
                100_000_001..=1_000_000_000 => "yellow",
                _ => "red",
            };
            let _human_size = human_bytes::human_bytes(e.1 as f64).color(color);
            return format!("{_human_size} {path}", path = e.0);
        })
        .collect();

    let selections = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt("Choose which node_modules to delete")
        .items(&names)
        .interact()
        .unwrap();

    let mut deleted = 0;

    if selections.is_empty() {
        println!("No node_modules selected. Aborting...");
    } else {
        println!("The following node_modules will be deleted:");
        for selection in &selections {
            println!("  {}", &names[*selection]);
        }
        if Confirm::new().with_prompt("Continue? ").interact().unwrap() {
            for selection in &selections {
                match remove_dir_all(&paths[*selection]) {
                    Ok(_) => deleted += &node_modules[*selection].1,
                    Err(e) => println!("Failed to delete {}. Error: {}", &names[*selection], e),
                }
            }

            println!("Freed  {} bytes", human_bytes::human_bytes(deleted as f64));
        } else {
            println!("Aborting...");
        }
    }
}

/* TODO: add coverage via https://github.com/marketplace/actions/rust-grcov*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::create_dir_all;
    use std::path::PathBuf;

    #[test]
    fn gets_home_directory() {
        let homedir = home_dir().unwrap();
        let home_dir = get_home_dir();
        assert_eq!(homedir.to_str().unwrap(), home_dir)
    }

    #[test]
    fn finds_node_modules_paths() {
        /* Create a new temporary directory */
        let temp = assert_fs::TempDir::new().unwrap();
        /* Create a node_modules with an empty package.json in it */
        let node_modules_path_str: PathBuf =
            [temp.to_str().unwrap(), "node_modules"].iter().collect();
        let node_modules_file_path_str: PathBuf =
            [node_modules_path_str.to_str().unwrap(), "package.json"]
                .iter()
                .collect();

        if let Err(e) = create_dir_all(&node_modules_path_str) {
            panic!("Error creating tests node_modules directory: {e}")
        }

        /* Assert that the tests node_modules directory exists */
        assert!(&node_modules_path_str.as_path().exists());

        let temp_path = temp.to_str().unwrap().to_owned();

        let mut paths = get_node_module_paths(temp_path.clone());

        /* Assert that we found the node_modules folder */
        assert_eq!(
            paths.first().unwrap().0,
            node_modules_path_str.to_str().unwrap()
        );

        /* Assert that it's empty */
        assert_eq!(paths.first().unwrap().1, 0);

        /* Add some data to the package.json file */
        let data = "hello";

        if let Err(e) = fs::write(&node_modules_file_path_str, data) {
            panic!("Error writing to temp file: {e}");
        };

        paths = get_node_module_paths(temp_path);

        /* Assert that we found the node_modules folder */
        assert_eq!(paths[0].0, node_modules_path_str.to_str().unwrap());

        let file_contents = fs::read_to_string(node_modules_file_path_str.as_path());
        assert_eq!(file_contents.unwrap(), data);

        let file_size = paths[0].1;
        assert_eq!(file_size, data.as_bytes().len() as u64);

        temp.close().unwrap();
    }
}
