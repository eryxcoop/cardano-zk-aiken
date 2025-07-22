use crate::circom_compiler::CircomCompiler;
use crate::tests::utils::create_sandbox_and_set_as_current_directory;
use serial_test::serial;
use std::fs;
use chumsky::primitive::Container;
use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;

#[test]
#[serial]
fn test_compiler_can_save_source_code_into_file() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let source_code = source_code_addition();
    let source_code_path = "test.circom";

    let mut compiler = CircomCompiler::from(source_code.clone());
    compiler
        .save_into_file(source_code_path.to_string())
        .unwrap();

    let stored_content = fs::read_to_string(&source_code_path).expect("No se pudo leer el archivo");
    assert_eq!(source_code, stored_content);
    assert_eq!(source_code_path, compiler.circom_source_code_path.unwrap());
}

#[test]
#[serial]
fn test_compiler_can_compile_the_generated_circom_component() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let circom_program_filename = "test.circom".to_string();
    let mut compiler = CircomCompiler::from(source_code_addition());
    compiler
        .save_into_file(circom_program_filename.clone())
        .unwrap();
    let random_seeds = ("asdasd", "dsadsa");

    compiler
        .create_verification_key(circom_program_filename, random_seeds)
        .unwrap();

    let stored_vk =
        fs::read_to_string("build/verification_key.json").expect("No se pudo leer el archivo");
    assert!(stored_vk.contains("protocol"));
}

#[test]
#[serial]
fn test_xxx_creates_proof_object() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let circom_program_filename = "test.circom".to_string();
    let mut compiler = CircomCompiler::from(source_code_addition());
    compiler
        .save_into_file(circom_program_filename.clone())
        .unwrap();
    fs::write("inputs.json", "{\"a\":\"1\", \"b\":\"2\", \"c\":\"3\"}").unwrap();

    let proof =
        CircomCompiler::generate_proof("test.circom", "my_verification_key.zkey", "inputs.json");

    assert_proof_is_valid(proof);
}

// Ideally we would run a verification using the proof and the circuit
// but this would slow down noticeably the tests
fn assert_proof_is_valid(proof: CompressedGroth16ProofBls12_381) {
    assert_eq!(96, proof.piA_as_byte_string().len());
    assert!(proof.piA_as_byte_string().chars().into_iter().all(|c| c.is_ascii_hexdigit()));
    assert_eq!(192, proof.piB_as_byte_string().len());
    assert!(proof.piB_as_byte_string().chars().into_iter().all(|c| c.is_ascii_hexdigit()));
    assert_eq!(96, proof.piC_as_byte_string().len());
    assert!(proof.piC_as_byte_string().chars().into_iter().all(|c| c.is_ascii_hexdigit()));
}
// ---------- AUX ---------- //

fn source_code_addition() -> String {
    r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main { public [a,b,c] } = Addition();"#
        .to_string()
}
