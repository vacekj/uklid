use assert_cmd::cargo::cargo_bin;
use assert_cmd::Command;
use expectrl::{spawn, ControlCode, Error, Regex};
use std::fs;
use std::fs::create_dir_all;
use std::io::BufRead;
use std::path::PathBuf;
use std::time::Duration;

use uklid::get_home_dir;

#[test]
fn interactive_cli() -> Result<(), Error> {
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

    /* Add some data to the package.json file */
    let data = "hello";

    if let Err(e) = fs::write(&node_modules_file_path_str, data) {
        panic!("Error writing to temp file: {e}");
    };

    let home_dir = get_home_dir();

    /* Spawn interactive CLI */
    let mut p = spawn("./target/debug/uklid").unwrap();
    p.set_expect_timeout(Some(Duration::from_secs(1)));
    let prompt = &format!("Where should I start searching? [{home_dir}]");
    p.expect(prompt).unwrap();
    p.send_line(&temp_path)?;
    p.expect(Regex(
        "Found 1 node_module folders with a total size of \\d",
    ))?;
    p.expect(Regex("Choose which node_modules to delete:"))?;
    p.expect(Regex(format!(".*\\[ \\].*{temp_path}/node_modules.*")))?;
    p.send_control(ControlCode::CarriageReturn)?;
    p.expect("No node_modules selected. Aborting...")?;

    assert!(&node_modules_path_str.as_path().exists());

    let mut p = spawn("./target/debug/uklid").unwrap();
    p.set_expect_timeout(Some(Duration::from_secs(1)));
    let prompt = &format!("Where should I start searching? [{home_dir}]");
    p.expect(prompt).unwrap();
    p.send_line(&temp_path)?;
    p.expect(Regex(
        "Found 1 node_module folders with a total size of \\d",
    ))?;
    p.expect(Regex("Choose which node_modules to delete:"))?;
    p.expect(Regex(format!(".*\\[ \\].*{temp_path}/node_modules.*")))?;
    p.send_control(ControlCode::Space)?;
    p.expect("> [x]")?;
    p.send_control(ControlCode::CarriageReturn)?;
    p.expect("The following node_modules will be deleted:")?;
    p.expect(Regex(format!(".*{temp_path}/node_modules.*")))?;
    p.expect("Continue?  [y/n]")?;
    p.send("y")?;
    p.expect(Regex(format!("Freed .* bytes")))?;

    temp.close().unwrap();
    Ok(())
}
