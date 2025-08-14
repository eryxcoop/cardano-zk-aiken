use crate::circom_circuit::CircomCircuit;
use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;
use crate::tests::utils::create_sandbox_and_set_as_current_directory;
use serial_test::serial;
use std::fs;
use crate::compiler::BUILD_DIR;

#[test]
#[serial]
fn test_circuit_can_generate_a_verification_key() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let circom_program_filename = "test.circom".to_string();
    fs::write(&circom_program_filename, source_code_addition()).unwrap();
    let circuit = CircomCircuit::from(circom_program_filename.clone());
    let random_seeds = ("asdasd", "dsadsa");

    circuit.generate_verification_key(random_seeds).unwrap();

    let stored_vk =
        fs::read_to_string(BUILD_DIR.to_string() + "verification_key.json").expect("No se pudo leer el archivo");
    assert!(stored_vk.contains("protocol"));
}

#[test]
#[serial]
fn test_circuit_can_generate_a_proof() {
    let circom_path = "test.circom";
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let circom_program_filename = circom_path.to_string();
    fs::write(&circom_program_filename, source_code_addition()).unwrap();
    fs::write("inputs.json", "{\"a\":\"1\", \"b\":\"2\", \"c\":\"3\"}").unwrap();
    let circom_circuit = CircomCircuit::from(circom_path.to_string());

    let proof = circom_circuit.generate_groth16_proof("my_verification_key.zkey", "inputs.json");

    assert_proof_is_valid(proof);
}

// Ideally we would run a verification using the proof and the circuit
// but this would slow down noticeably the tests
fn assert_proof_is_valid(proof: CompressedGroth16ProofBls12_381) {
    assert_eq!(96, proof.pi_a_as_byte_string().len());
    assert!(
        proof
            .pi_a_as_byte_string()
            .chars()
            .into_iter()
            .all(|c| c.is_ascii_hexdigit())
    );
    assert_eq!(192, proof.pi_b_as_byte_string().len());
    assert!(
        proof
            .pi_b_as_byte_string()
            .chars()
            .into_iter()
            .all(|c| c.is_ascii_hexdigit())
    );
    assert_eq!(96, proof.pi_c_as_byte_string().len());
    assert!(
        proof
            .pi_c_as_byte_string()
            .chars()
            .into_iter()
            .all(|c| c.is_ascii_hexdigit())
    );
}
// ---------- AUX ---------- //

fn source_code_addition() -> String {
    r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main { public [a,b,c] } = Addition();"#
        .to_string()
}
