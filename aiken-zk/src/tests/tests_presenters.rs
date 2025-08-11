use crate::circom_circuit::CircomCircuit;
use crate::presenter::compressed_groth16_proof_bls12_381_to_aiken_presenter::CompressedGroth16ProofBls12_381ToAikenPresenter;
use crate::presenter::meshjs_zk_redeemer_presenter::MeshJsZKRedeemerPresenter;
use crate::tests::utils::create_sandbox_and_set_as_current_directory;
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn test_aiken_proof_is_correctly_presented() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let circom_path = "my_program.circom";
    let verification_key_path = "my_verification_key.zkey";
    let inputs_path = "inputs.json";

    create_circom_and_inputs_file();

    let circuit = CircomCircuit::from(circom_path.to_string());
    let proof = circuit.generate_groth16_proof(verification_key_path, inputs_path);

    let aiken_presenter = CompressedGroth16ProofBls12_381ToAikenPresenter::new(proof.clone());

    let aiken_proof = aiken_presenter.present();
    let expected_presented_proof = format!(
        "Proof {{
\tpiA: #\"{}\",
\tpiB: #\"{}\",
\tpiC: #\"{}\",
}}",
        &proof.pi_a_as_byte_string(),
        &proof.pi_b_as_byte_string(),
        &proof.pi_c_as_byte_string()
    );

    assert_eq!(expected_presented_proof, aiken_proof);
}

#[test]
#[serial]
fn test_meshjs_library_is_correctly_presented() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let circom_path = "my_program.circom";
    let verification_key_path = "my_verification_key.zkey";
    let inputs_path = "inputs.json";

    create_circom_and_inputs_file();

    let circuit = CircomCircuit::from(circom_path.to_string());
    let proof = circuit.generate_groth16_proof(verification_key_path, inputs_path);
    let mesh_js_presenter = MeshJsZKRedeemerPresenter::new_for_proof(proof.clone());
    let presented_meshjs_library = mesh_js_presenter.present();

    let expected_presented_proof = format!(
        "\t\tmProof(
\t\t\t\"{}\",
\t\t\t\"{}\",
\t\t\t\"{}\",
\t\t),
",
        proof.pi_a_as_byte_string(),
        proof.pi_b_as_byte_string(),
        proof.pi_c_as_byte_string()
    );
    let expected_presented_meshjs_library = format!(
        "{}{}{}",
        meshjs_file_prefix(),
        expected_presented_proof,
        meshjs_file_suffix()
    );

    assert_eq!(expected_presented_meshjs_library, presented_meshjs_library);
}

fn create_circom_and_inputs_file() {
    fs::write("my_program.circom", circom_file()).expect("output file write failed");
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
