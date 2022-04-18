#![feature(path_try_exists)]
use colored::Colorize;
use dialoguer::{theme::SimpleTheme, Confirm, Input, MultiSelect};
use fs_extra::dir::get_size;
use home::home_dir;
use human_bytes::human_bytes;
use std::fs::remove_dir_all;
use std::path::Path;

use walkdir::WalkDir;

fn main() {
    let hd = home_dir();
    let home_directory = match hd.as_ref() {
        None => "/",
        Some(dir) => dir.to_str().unwrap(),
    };

    let starting_directory: String = Input::new()
        .with_prompt("Where should I start searching?")
        .default(home_directory.into())
        .interact_text()
        .unwrap();

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

    let _count = node_modules.len();
    let _total_size = human_bytes(node_modules.iter().fold(0, |acc, x| acc + x.1) as f64);

    println!("Found {_count} node_module folders with a total size of {_total_size}");

    let names: Vec<String> = node_modules
        .iter()
        .map(|e| {
            let color = match e.1 {
                1..=100_000_000 => "green",
                100_000_001..=1_000_000_000 => "yellow",
                _ => "red",
            };
            let _human_size = human_bytes(e.1 as f64).color(color);
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
                if remove_dir_all(&names[*selection]).is_ok() {
                    deleted += &node_modules[*selection].1;
                } else {
                    println!("Failed to delete {}", &names[*selection]);
                }
            }

            println!("Freed  {} bytes", human_bytes(deleted as f64));
        } else {
            println!("Aborting...");
        }
    }
}
