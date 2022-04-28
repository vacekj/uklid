use std::fs::create_dir_all;
use std::path::PathBuf;
use uklid::get_node_module_paths;

#[test]
fn it_finds_paths() {
    /* Create a new temporary directory */
    let temp = assert_fs::TempDir::new().unwrap();
    /* Create a node_modules with an empty package.json in it */
    let node_modules_path_str: PathBuf = [temp.to_str().unwrap(), "node_modules", "package.json"]
        .iter()
        .collect();

    if let Err(e) = create_dir_all(&node_modules_path_str) {
        println!("Error creating test node_modules directory: {}", e)
    }

    /* Assert that the test node_modules directory exists */
    assert!(&node_modules_path_str.as_path().exists());

    let paths = get_node_module_paths(temp.to_str().unwrap().to_owned());

    let test_path: PathBuf = [temp.to_str().unwrap(), "node_modules"].iter().collect();

    assert_eq!(paths.first().unwrap().0, test_path.to_str().unwrap());

    assert_eq!(paths.first().unwrap().1, 0);
}
