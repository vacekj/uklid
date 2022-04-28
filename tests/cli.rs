use assert_cmd::Command;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;

#[test]
fn gets_files() {
    /* Create a new temporary directory */
    let temp = assert_fs::TempDir::new().unwrap();
    /* Create a node_modules with an empty package.json in it */
    let node_modules_path_str: PathBuf = [temp.to_str().unwrap(), "node_modules"].iter().collect();
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

    let dry_run_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--path".to_string(), temp_path.clone(), "--dry".to_owned()])
        .unwrap();

    let dry_std_out = std::str::from_utf8(&dry_run_cmd.stdout).unwrap();

    /* Finds node_modules and dry runs */
    assert_eq!(dry_std_out,
               format!("The following node_modules will be deleted:\n  {temp_path}/node_modules\nArgument dry provided, no directories were deleted.\n"));

    /* Nothing was deleted */
    assert!(node_modules_path_str.as_path().exists());

    /* Add some data to the package.json file */
    let data = "hello";

    if let Err(e) = fs::write(&node_modules_file_path_str, data) {
        panic!("Error writing to temp file: {e}");
    };

    let hot_run_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--path".to_string(), temp_path.clone()])
        .unwrap();

    let hot_run_stdout = std::str::from_utf8(&hot_run_cmd.stdout).unwrap();

    /* Finds node_modules and deletes files */
    assert_eq!(hot_run_stdout,
               format!("The following node_modules will be deleted:\n  {temp_path}/node_modules\nFreed  5 B bytes\n"));

    assert!(!node_modules_file_path_str.exists());
    assert!(!node_modules_path_str.exists());

    temp.close().unwrap();
}
