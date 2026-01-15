use crate::circom_circuit::CircomCircuit;
use crate::tests::circom_component_factory::addition_custom_circom_template_and_component;
use crate::tests::utils::{create_sandbox_and_set_as_current_directory, manifest_path};
use serial_test::serial;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::Command;
use std::{fs, io};

pub const BUILD_COMMAND: &str = "build";

#[test]
#[serial]
fn test_user_can_convert_aiken_with_offchain_to_valid_aiken() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let output_path = "validators/output.ak";
    create_original_aiken_file();

    Command::new(aiken_zk_binary_path)
        .arg(BUILD_COMMAND)
        .arg(source_aiken_filename())
        .arg(output_path)
        .output()
        .unwrap();

    let compilation_result = Command::new("aiken").arg(BUILD_COMMAND).output().unwrap();
    let file = File::open(output_path).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let expected_line_replacement = "zk_verify_or_fail(redeemer, [Single(b), Single(10)])";
    let expected_line_declaration = "fn zk_verify_or_fail(";
    assert!(lines[19].contains(expected_line_replacement));
    assert!(lines[33].contains(expected_line_declaration));
    assert!(Path::new("verification_key.zkey").exists());
    assert!(Path::new("output.circom").exists());

    assert!(compilation_result.status.success());
    assert!(Path::new("plutus.json").exists());
}

#[test]
#[serial]
fn test_user_can_convert_aiken_with_custom_circom_offchain_to_valid_aiken() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let output_path = "validators/output.ak";
    fs::write(
        "./addition.circom",
        addition_custom_circom_template_and_component(),
    )
    .unwrap();
    fs::write(
        source_aiken_filename(),
        original_aiken_code_with_custom_token(),
    )
    .expect("output file write failed");

    Command::new(aiken_zk_binary_path)
        .arg(BUILD_COMMAND)
        .arg(source_aiken_filename())
        .arg(output_path)
        .output()
        .unwrap();

    let compilation_result = Command::new("aiken").arg(BUILD_COMMAND).output().unwrap();
    let file = fs::read_to_string(output_path).unwrap();
    let expected_line_replacement = "zk_verify_or_fail(redeemer, [Single(b), Single(5)])";
    let expected_line_declaration = "fn zk_verify_or_fail(";
    assert!(file.contains(expected_line_replacement));
    assert!(file.contains(expected_line_declaration));
    assert!(Path::new("verification_key.zkey").exists());
    assert!(Path::new("addition.circom").exists());

    assert!(compilation_result.status.success());
    assert!(Path::new("plutus.json").exists());
}

#[test]
#[serial]
fn test_user_can_generate_an_aiken_proof() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let circom_path = "my_program.circom";
    let verification_key_path = "verification_key.zkey";
    let inputs_path = "inputs.json";
    let output_path = "proof.ak";
    create_circom_and_inputs_file(inputs_path);

    Command::new(aiken_zk_binary_path)
        .arg("prove")
        .arg("aiken")
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

#[test]
#[serial]
fn test_user_can_generate_a_meshjs_proof() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let aiken_zk_binary_path = manifest_path() + "/target/debug/aiken-zk";
    let circom_path = "my_program.circom";
    let verification_key_path = "verification_key.zkey";
    let inputs_path = "inputs.json";
    let output_path = "zk_redeemer.ts";
    create_circom_and_inputs_file(inputs_path);

    Command::new(aiken_zk_binary_path)
        .arg("prove")
        .arg("meshjs")
        .arg(circom_path)
        .arg(verification_key_path)
        .arg(inputs_path)
        .arg(output_path)
        .output()
        .unwrap();

    let file = File::open(output_path).unwrap();
    let mut reader = io::BufReader::new(file);

    assert_text_matches(&mut reader, meshjs_file_prefix());

    assert_line_matches(&mut reader, "\t\tmProof(\n");

    assert_proof_component_format_is_correct(&mut reader, 96);
    assert_proof_component_format_is_correct(&mut reader, 192);
    assert_proof_component_format_is_correct(&mut reader, 96);

    assert_line_matches(&mut reader, "\t\t),\n");

    assert_text_matches(&mut reader, meshjs_file_suffix());
}

fn assert_line_matches(reader: &mut BufReader<File>, expected_line: &str) {
    let mut line_to_assert = String::new();
    reader.read_line(&mut line_to_assert).unwrap();
    assert_eq!(expected_line, line_to_assert);
}

fn assert_text_matches(reader: &mut BufReader<File>, expected_text: String) {
    let mut buffer = vec![0u8; expected_text.len()]; // un buffer de N bytes
    let bytes_read = reader.read(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&buffer[..bytes_read]);

    assert_eq!(expected_text, text);
}

fn assert_proof_component_format_is_correct(
    reader: &mut BufReader<File>,
    proof_component_length_as_byte_string: usize,
) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let expected_prefix = "\t\t\t\"";
    let expected_suffix = "\",\n";

    let prefix = &line[..expected_prefix.len()];
    let pi_n =
        &line[expected_prefix.len()..expected_prefix.len() + proof_component_length_as_byte_string];
    let suffix = &line[expected_prefix.len() + proof_component_length_as_byte_string..];

    assert_eq!(expected_prefix.to_string(), prefix);
    assert!(pi_n.chars().into_iter().all(|c| c.is_ascii_hexdigit()));
    assert_eq!(expected_suffix.to_string(), suffix);
}

fn meshjs_file_prefix() -> String {
    r#"import {MConStr} from "@meshsdk/common";
import {Data, mConStr0} from "@meshsdk/core";

type Proof = MConStr<any, string[]>;

type ZKRedeemer = MConStr<any, Data[] | Proof[]>;

function mProof(piA: string, piB: string, piC: string): Proof {
    if (piA.length != 96 || piB.length != 192 || piC.length != 96) {
        throw new Error("Wrong proof");
    }

    return mConStr0([piA, piB, piC]);
}

export function mZKRedeemer(redeemer: Data): ZKRedeemer {
    return mConStr0([redeemer, proofs()]);
}

function proofs(): Proof[] {
    return [
"#
    .to_string()
}

fn meshjs_file_suffix() -> String {
    r#"    ];
}
"#
    .to_string()
}

fn source_aiken_filename() -> &'static str {
    "original_aiken_code.ak"
}

fn create_original_aiken_file() {
    fs::write(source_aiken_filename(), original_aiken_code()).expect("output file write failed");
}

fn create_circom_and_inputs_file(inputs_path: &str) {
    let circom_path = "my_program.circom";
    fs::write(circom_path, circom_file()).unwrap();
    let circom_compiler = CircomCircuit::from(circom_path.to_string());
    circom_compiler
        .generate_verification_key(("a", "b"))
        .unwrap();

    fs::write(inputs_path, inputs_json()).expect("output file write failed");
}

fn circom_file() -> String {
    r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main { public [first_addend,second_addend,sum] } = Addition();"#
        .to_string()
}

fn inputs_json() -> String {
    r#"{"first_addend": "3", "second_addend": "7", "sum": "10"}"#.to_string()
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
    expect _zk_redeemer = offchain addition(priv, b, 10)
    True
  }

  else(_) {
    fail
  }
}
"#
    .to_string()
}

fn original_aiken_code_with_custom_token() -> String {
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
    expect _zk_redeemer = offchain custom("addition.circom", [b, 5])
    True
  }

  else(_) {
    fail
  }
}
"#
    .to_string()
}
