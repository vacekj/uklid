use clap::Parser;
use std::fs::remove_dir_all;
use uklid::{get_node_module_paths, interactive_clean};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to start recursive search for node_modules from
    #[clap(short, long)]
    path: Option<String>,

    /// Don't delete anything, only print found directories
    #[clap(short, long)]
    dry: bool,
}

fn delete_modules(modules: Vec<(String, u64)>) -> u64 {
    let mut deleted = 0;
    for module in &modules {
        if remove_dir_all(module.0.clone()).is_ok() {
            deleted += module.1;
        } else {
            println!("Failed to delete {}", module.0.clone());
        }
    }
    deleted
}

fn main() {
    let args = Cli::parse();

    /* If the user specified a path, skip the interactive CLI */
    if let Some(path) = args.path.as_deref() {
        let modules = get_node_module_paths(path.to_string());

        println!("The following node_modules will be deleted:");
        for selection in &modules {
            println!("  {}", selection.0);
        }

        if args.dry {
            println!("Argument dry provided, no directories were deleted.");
            return;
        }

        let deleted_bytes = delete_modules(modules);

        println!(
            "Freed  {} bytes",
            human_bytes::human_bytes(deleted_bytes as f64)
        );
    } else {
        /* User did not specify a path, launch interactive CLI */
        interactive_clean();
    }
}
