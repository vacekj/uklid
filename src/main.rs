use dialoguer::{theme::SimpleTheme, Confirm, MultiSelect};
use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use std::borrow::Cow;
use std::fs::remove_dir_all;
use std::ops::Deref;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use walkdir::WalkDir;

fn main() {
    let mut node_modules: Vec<(String, u64)> = WalkDir::new("/Users/vacekj/Programming")
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
            let owned_e = e.unwrap().to_owned();
            return (
                owned_e.path().to_str().unwrap().to_owned(),
                get_size(owned_e.path()).unwrap_or(0),
            );
        })
        .collect();

    node_modules.sort_unstable_by_key(|k| k.1);
    node_modules.reverse();

    let count = node_modules.len();
    let total_size = human_bytes(node_modules.iter().fold(0, |acc, x| acc + x.1) as f64);

    println!("Found {count} node_module folders with a total size of {total_size}");
    for dir in &node_modules {
        println!("{0} {1}", human_bytes(dir.1 as f64), dir.0);
    }

    let names: Vec<String> = node_modules.iter().map(|e| e.0.clone()).collect();

    let selections = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt("Choose which node_modules to delete")
        .items(&names)
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("No node_modules selected. Aborting...");
        return;
    } else {
        println!("The following node_modules will be deleted:");
        for selection in &selections {
            println!("  {}", &names[*selection]);
        }
        if Confirm::new().with_prompt("Continue? ").interact().unwrap() {
            for selection in &selections {
                remove_dir_all(&names[*selection]);
            }
        } else {
            println!("Aborting...");
            return;
        }
    }
}
