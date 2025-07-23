use crate::circom_compiler::CircomCompiler;
use crate::tests::utils::{create_sandbox_and_set_as_current_directory, manifest_path};
use serial_test::serial;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::Command;
use std::{fs, io};

#[test]
#[serial]
fn test_user_can_convert_aiken_with_offchain_to_valid_aiken() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let output_path = "validators/output.ak";
    create_original_aiken_file();

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
    assert!(Path::new("verification_key.zkey").exists());
    assert!(Path::new("output.circom").exists());

    assert!(compilation_result.status.success());
    assert!(Path::new("plutus.json").exists());
}

#[test]
#[serial]
fn test_user_can_generate_a_proof() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let circom_path = "my_program.circom";
    let verification_key_path = "verification_key.zkey";
    let inputs_path = "inputs.json";
    let output_path = "proof.ak";
    create_circom_and_inputs_file();

    Command::new(aiken_zk_binary_path)
        .arg("prove")
        .arg(circom_path)
        .arg(verification_key_path)
        .arg(inputs_path)
        .arg(output_path)
        .output()
        .unwrap();

    let file = File::open(output_path).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    // todo: check verification
    assert_eq!("Proof {", lines[0]);
    assert!(lines[1].contains("piA: #"));
    assert!(lines[2].contains("piB: #"));
    assert!(lines[3].contains("piC: #"));
    assert_eq!("}", lines[4]);
}

fn source_aiken_filename() -> &'static str {
    "original_aiken_code.ak"
}

fn create_original_aiken_file() {
    fs::write(source_aiken_filename(), original_aiken_code()).expect("output file write failed");
}

fn create_circom_and_inputs_file() {
    let circom_path = "my_program.circom";
    fs::write(circom_path, circom_file()).unwrap();
    let mut circom_compiler = CircomCompiler::from(circom_path.to_string());

    circom_compiler
        .create_verification_key(("a", "b"))
        .unwrap();

    fs::write("inputs.json", inputs_json()).expect("output file write failed");
}

fn circom_file() -> String {
    r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main { public [a,b,c] } = Addition();"#
        .to_string()
}

fn inputs_json() -> String {
    r#"{"a": "3", "b": "7", "c": "10"}"#.to_string()
}

fn original_aiken_code() -> String {
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
"#
    .to_string()
}
