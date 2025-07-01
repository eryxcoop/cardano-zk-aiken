use crate::tests::utils::{create_sandbox_and_set_as_current_directory, manifest_path};
use serial_test::serial;
use std::fs::File;
use std::{fs, io};
use std::io::{BufRead, ErrorKind};
use std::path::Path;
use std::process::Command;

#[test]
#[serial]
fn test_user_can_convert_aiken_with_offchain_to_valid_aiken() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let output_path = "validators/output.ak";
    create_original_aiken_file_and_inputs();

    Command::new(aiken_zk_binary_path)
        .arg("build")
        .arg(source_aiken_filename())
        .arg(output_path)
        .output()
        .unwrap();

    let compilation_result = Command::new("aiken").arg("build").output().unwrap();
    let file = File::open(output_path).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let expected_line_replacement = "zk_verify_or_fail(redeemer, [b, 10])";
    let expected_line_declaration = "fn zk_verify_or_fail(";
    assert!(lines[19].contains(expected_line_replacement));
    assert!(lines[28].contains(expected_line_declaration));
    assert!(Path::new("my_verification_key.zkey").exists());
    assert!(Path::new("output.circom").exists());

    assert!(compilation_result.status.success());
    assert!(Path::new("plutus.json").exists());
}

fn source_aiken_filename() -> &'static str {
    "original_aiken_code.ak"
}

fn create_original_aiken_file_and_inputs() {
    fs::create_dir("validators").or_else(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(error)
        }
    }).expect("Couldnt create dir");
    fs::write(source_aiken_filename(), original_aiken_code()).expect("output file write failed");
    // fs::write("inputs.json", inputs_json()).expect("output file write failed");
}

fn _inputs_json() -> String {
    r#"{"a": "3", "b": "7", "c": "10"}"#.to_string()
}

fn original_aiken_code() -> String{
    r#"use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

type Redeemer = Void

validator example {
  spend(
    datum: Option<Int>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(b) = datum
    expect _zk_redeemer = offchain addition(priv a, b, 10)
    True
  }

  else(_) {
    fail
  }
}
"#.to_string()
}