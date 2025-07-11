use crate::aiken_zk_compiler::{AikenZkCompiler, Groth16CompressedData};
use crate::tests::aiken_program_factory::{
    aiken_template_with_body_and_verify_definition, verify_declaration,
};
use crate::tests::utils::create_sandbox_and_set_as_current_directory;
use serial_test::serial;
use std::fs::File;
use std::io::BufRead;
use std::{fs, io};

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_public_variables_by_the_corresponding_function_and_call() {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(pub a, pub b, pub c)",
        "zk_verify_or_fail(redeemer, [a, b, c])",
        addition_all_public_vk_compressed(),
        3,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_private_variables_by_the_corresponding_function_and_call()
{
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(priv a, priv b, priv c)",
        "zk_verify_or_fail(redeemer, [])",
        addition_all_private_vk_compressed(),
        0,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_mixed_variables_by_the_corresponding_function_and_call() {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(priv a, b, pub c)",
        "zk_verify_or_fail(redeemer, [b, c])",
        addition_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(priv a, 4, pub b)",
        "zk_verify_or_fail(redeemer, [4, b])",
        addition_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_subtraction_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain subtraction(priv a, 4, pub b)",
        "zk_verify_or_fail(redeemer, [4, b])",
        addition_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_multiplication_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain multiplication(priv a, 4, pub b)",
        "zk_verify_or_fail(redeemer, [4, b])",
        multiplication_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_fibonacci_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain fibonacci(priv a, b, 5, pub c)",
        "zk_verify_or_fail(redeemer, [b, c])",
        fibonacci_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_if_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain if(a, b, priv c, priv d)",
        "zk_verify_or_fail(redeemer, [a, b])",
        if_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_assert_eq_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain assert_eq(priv a, b)",
        "zk_verify_or_fail(redeemer, [b])",
        assert_eq_mixed_visibility_vk_compressed(),
        1,
    );
}

#[test]
#[serial]
fn test_tool_can_generate_proof() {
    let _temporal_directory = create_sandbox_and_set_as_current_directory();
    let circom_path = "my_program.circom";
    let verification_key_path = "my_verification_key.zkey";
    let inputs_path = "inputs.json";

    let output_path = "proof.ak";
    create_circom_and_inputs_file();

    AikenZkCompiler::generate_aiken_proof(
        circom_path,
        verification_key_path,
        inputs_path,
        output_path,
    );

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

fn test_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
    original: &str,
    replacement: &str,
    vk_compressed_data: Groth16CompressedData,
    n_public_inputs: usize,
) {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let aiken_src = aiken_template_with_body_and_verify_definition("", original, "");
    let output_filename = "my_program".to_string();
    let random_seeds = ("asdasd", "dsadsa");

    let aiken_zk_src = AikenZkCompiler::apply_modifications_to_src_for_token(
        aiken_src,
        output_filename,
        random_seeds,
    );

    let verify_declaration = verify_declaration(n_public_inputs, vk_compressed_data);
    let expected_aiken_src = aiken_template_with_body_and_verify_definition(
        import_header(),
        replacement,
        &verify_declaration,
    );

    assert_eq!(
        without_delta(expected_aiken_src),
        without_delta(aiken_zk_src)
    );
}

fn import_header() -> &'static str {
    "use aiken/collection/list\nuse ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}\n"
}

fn addition_all_public_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "b42a4610c5c2722df0cae5b696d0e212dd41e471a5246217751ae313dceba2b4d25c1be296ee8e00454027b7c4a45208".to_string(),
            "87ef7b539de25c06493f7cd054a78da2819084b7027038d28b31fe88ce0b833f243723fbd9c4e502a3d0c2246aa69560".to_string(),
            "a680399022e0bd33fa72396626b4bfc5d1d42e6d9207f3bc64f9fd26a32e5d150ba63a7c28d61db724d362bb9cf96680".to_string(),
            "87ac4ff5d2863dd744e3ad397dfde8fe657c09c9c059e25ab8f37b85822eb8f34604d7ca2fe2622d1003ed258319bbf2".to_string(),
        ],
    }
}

fn multiplication_mixed_visibility_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "aba434215d34bebf3100b82fb68eaa69328cc6431a26ecc8ef81bffced149a5f7e193587789a1a0c6745b3e963c1989e".to_string(),
            "8f134dfdb298b8bbda90184813301698e6eb3cf489e66f155f6f2660ee60813b0d1f7227db0fa5906f6d52f2263c5bd4".to_string(),
            "8cc7b9ce6dbd0e58188384ccabc6255ecee8e7756de001c92f793a0eb1be167b4a9b9f18a52560d16662619ba6fe57f8".to_string(),
        ],
    }
}

fn addition_all_private_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "b8fcac9bb8eebddd4daf43519eb65d952436f5e98be287e246d70fc27f267379e132a156f6a4a742ece62fbb7c5e220d".to_string(),
        ],
    }
}

fn addition_mixed_visibility_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "af1ca9d68a382928932cd5f1a3dde62489556f42da0c24e6d11191c1b187f147a206da840166e28f1ae73edee0c8b912".to_string(),
            "aba434215d34bebf3100b82fb68eaa69328cc6431a26ecc8ef81bffced149a5f7e193587789a1a0c6745b3e963c1989e".to_string(),
            "87db49b3c35ae1d3f5b767abf48ca5b73d17c81ad5c50419386a09415e7eba5b7bf50e5d3d2976ec11c31ad4f2ec3477".to_string(),
        ],
    }
}

fn fibonacci_mixed_visibility_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "af1ca9d68a382928932cd5f1a3dde62489556f42da0c24e6d11191c1b187f147a206da840166e28f1ae73edee0c8b912".to_string(),
            "aba434215d34bebf3100b82fb68eaa69328cc6431a26ecc8ef81bffced149a5f7e193587789a1a0c6745b3e963c1989e".to_string(),
            "87db49b3c35ae1d3f5b767abf48ca5b73d17c81ad5c50419386a09415e7eba5b7bf50e5d3d2976ec11c31ad4f2ec3477".to_string(),
        ],
    }
}

fn if_mixed_visibility_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "b2bc654c791b51b74caa10f4e71887609b45b7c8a9db92ad86a7067ca9899599a62406ae57ac26db27b456171ffc3198".to_string(),
            "93575ed08ccd19b2ade4963c459840e35da43198f88b5289edc88b463b958f681139329fb3c9b79ac22c2015e85ec84b".to_string(),
            "b967ef1ddaade3245e27fb745d0f6dfd4d9fcef20ee76ac0eef0afc20eb4122a5e72b6403fb20d73d96ee8bb62a210bb".to_string(),
        ],
    }
}

fn assert_eq_mixed_visibility_vk_compressed() -> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "b8fcac9bb8eebddd4daf43519eb65d952436f5e98be287e246d70fc27f267379e132a156f6a4a742ece62fbb7c5e220d".to_string(),
            "99f6c043cc37650767938eb567327aca0e82fb1dcab833778a6b8d5c8d13a8f53d784e7dfbcba6d3c71b57b908530048".to_string(),
        ],
    }
}

fn without_delta(final_program: String) -> String {
    let mut result = String::new();
    let mut remainder: &str = &final_program;

    let start_idx = remainder.find(r#"vkDelta: #""#).unwrap();
    let (before, rest) = remainder.split_at(start_idx);
    result.push_str(before);

    let end_idx = rest[13..].find('"').unwrap();
    remainder = &rest[13 + end_idx + 1..];

    result.push_str(remainder);
    result
}
