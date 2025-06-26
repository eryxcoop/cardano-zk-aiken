use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::tests::utils::{create_sandbox_and_set_as_current_directory, manifest_path};
use serial_test::serial;
use std::process::Command;

#[test]
#[serial]
fn test_user_can_convert_aiken_with_offchain_to_valid_aiken() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let binary_path = manifest_path() + "/target/debug/aiken-zk";
    let output = Command::new(binary_path)
        .arg("original_aiken_code.ak")
        .arg("output.ak")
        .output()
        .unwrap();
    println!("{:?}", String::from_utf8_lossy(&output.stdout));

    // leer el archivo y buscar los strings
    let file = File::open("output.ak").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>().unwrap();
    let expected_line_replacement = "zk_verify_or_fail(redeemer, [b, 10])";
    let expected_line_declaration = "fn zk_verify_or_fail(";

    assert!(lines[19].contains(expected_line_replacement));
    assert!(lines[28].contains(expected_line_declaration));
}
