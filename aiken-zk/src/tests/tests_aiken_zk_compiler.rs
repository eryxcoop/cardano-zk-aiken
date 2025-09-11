use crate::compiler::aiken_zk_compiler::{AikenZkCompiler, Groth16CompressedData};
use crate::tests::aiken_program_factory::{
    aiken_template_with_body_and_verify_definition, verify_declaration,
};
use crate::tests::circom_component_factory::{addition_custom_circom_template_and_component, indexing_custom_circom_template_and_component};
use crate::tests::utils::create_sandbox_and_set_as_current_directory;
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn test_replaces_addition_of_public_variables_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(pub a, pub b, pub c)",
        "zk_verify_or_fail(redeemer, [Single(a), Single(b), Single(c)])",
        addition_all_public_vk_compressed(),
        3,
    );
}

#[test]
#[serial]
fn test_replaces_addition_of_private_variables_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(priv, priv, priv)",
        "zk_verify_or_fail(redeemer, [])",
        addition_all_private_vk_compressed(),
        0,
    );
}

#[test]
#[serial]
fn test_replaces_addition_of_mixed_variables_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(priv, b, pub c)",
        "zk_verify_or_fail(redeemer, [Single(b), Single(c)])",
        addition_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_addition_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
{
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain addition(priv, 4, pub b)",
        "zk_verify_or_fail(redeemer, [Single(4), Single(b)])",
        addition_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_subtraction_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain subtraction(priv, 4, pub b)",
        "zk_verify_or_fail(redeemer, [Single(4), Single(b)])",
        addition_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_multiplication_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
 {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain multiplication(priv, 4, pub b)",
        "zk_verify_or_fail(redeemer, [Single(4), Single(b)])",
        multiplication_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_fibonacci_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
{
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain fibonacci(priv, b, 5, pub c)",
        "zk_verify_or_fail(redeemer, [Single(b), Single(c)])",
        fibonacci_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_if_of_mixed_variables_and_constants_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain if(a, b, priv, priv)",
        "zk_verify_or_fail(redeemer, [Single(a), Single(b)])",
        if_mixed_visibility_vk_compressed(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_assert_eq_of_mixed_variables_and_constants_by_the_corresponding_function_and_call()
{
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain assert_eq(priv, b)",
        "zk_verify_or_fail(redeemer, [Single(b)])",
        get_compressed_verification_key_from_assert_eq_circuit_with_mixed_visibility(),
        1,
    );
}

#[test]
#[serial]
fn test_replaces_sha256_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain sha256(5, in, out)",
        "zk_verify_or_fail(redeemer, [Many(in), Many(out)])",
        get_compressed_verification_key_from_sha256_circuit_with_mixed_visibility(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_poseidon_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain poseidon(5, in, out)",
        "zk_verify_or_fail(redeemer, [Many(in), Single(out)])",
        get_compressed_verification_key_from_poseidon_circuit_with_mixed_visibility(),
        2,
    );
}

#[test]
#[serial]
fn test_replaces_merkle_tree_checker_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain merkle_tree_checker(3, leaf, root, path_elements, path_indices)",
        "zk_verify_or_fail(redeemer, [Single(leaf), Single(root), Many(path_elements), Many(path_indices)])",
        get_compressed_verification_key_from_merkle_tree_checker_circuit_with_mixed_visibility(),
        4,
    );
}

#[test]
#[serial]
fn test_replaces_polynomial_evaluations_by_the_corresponding_function_and_call() {
    assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
        "offchain polynomial_evaluations(6, coefficients, 9, pub domain, pub evaluations)",
        "zk_verify_or_fail(redeemer, [Many(coefficients), Many(domain), Many(evaluations)])",
        get_compressed_verification_key_from_polynomial_evaluations(),
        3,
    );
}

#[test]
#[serial]
fn test_replaces_custom_circom_by_the_corresponding_function_and_call() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    fs::write(
        "./test.circom",
        addition_custom_circom_template_and_component(),
    )
    .unwrap();
    let aiken_src = aiken_template_with_body_and_verify_definition(
        "",
        "offchain custom(\"test.circom\", [a, 5])",
        "",
    );
    let output_filename = "my_program".to_string();
    let random_seeds = ("asdasd", "dsadsa");

    let aiken_zk_src = AikenZkCompiler::apply_modifications_to_src_for_token(
        aiken_src,
        output_filename,
        random_seeds,
    );

    let expected_aiken_src = aiken_template_with_body_and_verify_definition(
        import_header(),
        "zk_verify_or_fail(redeemer, [Single(a), Single(5)])",
        &verify_declaration(2, addition_custom_circom_vk_compressed()),
    );

    assert_eq!(
        without_delta(expected_aiken_src),
        without_delta(aiken_zk_src)
    );
}

#[test]
#[serial]
fn test_replaces_custom_circom_with_list_variable_by_the_corresponding_function_and_call() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    fs::write(
        "./test.circom",
        indexing_custom_circom_template_and_component(),
    )
    .unwrap();
    let aiken_src = aiken_template_with_body_and_verify_definition(
        "",
        "offchain custom(\"test.circom\", [l, idx, val])",
        "",
    );
    let output_filename = "my_program".to_string();
    let random_seeds = ("asdasd", "dsadsa");

    let aiken_zk_src = AikenZkCompiler::apply_modifications_to_src_for_token(
        aiken_src,
        output_filename,
        random_seeds,
    );

    let expected_aiken_src = aiken_template_with_body_and_verify_definition(
        import_header(),
        "zk_verify_or_fail(redeemer, [Many(l), Single(idx), Single(val)])",
        &verify_declaration(3, addition_custom_circom_vk_compressed()),
    );

    assert_eq!(
        without_delta(expected_aiken_src),
        without_delta(aiken_zk_src)
    );
}

#[test]
#[serial]
#[should_panic(expected = "Amount of public inputs doesnt match")]
fn test_custom_circom_should_fail_if_amount_of_public_inputs_doesnt_match() {
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    fs::write(
        "./test.circom",
        addition_custom_circom_template_and_component(),
    )
    .unwrap();
    let aiken_src = aiken_template_with_body_and_verify_definition(
        "",
        "offchain custom(\"test.circom\", [a, 5, b])",
        "",
    );
    let output_filename = "my_program".to_string();
    let random_seeds = ("asdasd", "dsadsa");

    AikenZkCompiler::apply_modifications_to_src_for_token(aiken_src, output_filename, random_seeds);
}

fn assert_compiler_can_replace_keyword_by_the_corresponding_function_and_call(
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

fn get_compressed_verification_key_from_assert_eq_circuit_with_mixed_visibility()
-> Groth16CompressedData {
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

fn get_compressed_verification_key_from_sha256_circuit_with_mixed_visibility()
-> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
                "808b7562a2f964e83c57de1060c477b856f64bfc96e7f1770efe8caa655299f22bab07e41dccf6d3e0de2f5edfdafefa".to_string(),
                "84075b27ca722a33ab0fe060cc578269518c4494619610bd5460b233bb6b91da4687403d9fb40dbe8e74104ae4a3b515".to_string(),
                "b505885ea8fa30e0c46cf1f2723a3b7b9d8adf1cd6a80e1b4fbcb8a866383624a64b2c74bd1de2793d5eb43765b7d28c".to_string(),
                "a7af0992b3d5660b9da8443435e7edb09c19e178dfcaf2651624e22b4d730a1a8118d1394821da83801f2138cecaecd0".to_string(),
                "819c281ed03ce1da11bbfc9607736152995ab1e49a14114bc8171afd38ee6f4e4bbef7bb368f84c6606a6d3994dbe853".to_string(),
                "a72febc464be8e93c13fc42ba586749bc2c656c2ae52c6b8ddf83d3247ac3758e0d6e5577f43afac4d86decb2066d45d".to_string(),
                "907321f1be9b780752f14501c2569cfe1ceb7149d85ef9515c2bd65a386299fa8a7d165cfe770974004850375cb84aa7".to_string(),
                "8cb17594fd5a9d34e79651f62a425a81fe56fb15a0b6b512a5b4c9f2cb9f60864d851f0f32548e7a97297beb6fd9489d".to_string(),
        ],
    }
}

fn get_compressed_verification_key_from_poseidon_circuit_with_mixed_visibility()
-> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "9339935d9dfa00b3e8bc8ca344227609c2abd8141bd75b15ee5c36f49fae603f6a4216e0be095b8970a706548ffc2807".to_string(),
            "b132fba8fd86e2769f9b99abdbcb9692970558d3a5cc6792eecc19e5ff0778cc3d3ee89039d5954f970ac6a7bc36ee49".to_string(),
            "884fdd6dc6dd2011aa4b980261cc536d0990c0e312e00c906020a4306926de451337d2f2b6ee45688efd3c6eeb357d82".to_string(),
            "95af44a74fb1404be22f66013c578d6ae65c3c1a4ca57936a6b811cfcecc7bc96b79eb308518b778de9793e61712cc57".to_string(),
            "ace3140f231d3cb91f3277123338c12a4d4cf456ef88599962e070723b03301c6d53d90454081d1bddba7df5ed0c1a90".to_string(),
            "a5118a34bb1c8efde89fb9a7da7251b7bbd0c2742bfdd4018fd76b2580ba5ab66cb6b08e2054a3e44d02160050d4a71d".to_string(),
            "aacd9c27228556daaa229566006b5f696ba6baf022f2c4e0b5903fc84440ed65b46763fc3c3801f8c21cc651345dd341".to_string(),
        ],
    }
}

fn get_compressed_verification_key_from_merkle_tree_checker_circuit_with_mixed_visibility()
-> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
            "97164a011bc26a95b15cba9015251637c93749296e14f6ef23665fc30be3cc70de1b4a45164977e7b223f5e646b14977".to_string(),
            "ad4651cc144d2009c864d2887d97219c232e64e1f203ea8088a78c8d5988cdd0ed3c7e2f95a05dd275412a7112499593".to_string(),
            "8eea02bfd57d712ad2a9408f4ebc91fb98ce08c539ec45610f07b8ebcb1185aaa256dde0203a9c517b36bb02a95433c4".to_string(),
            "83da80ea63183933a07cc3f1009b1e0693e281ac6b353c7371fa30ff8758e2071133e910784c62dbef0c243e2bec0036".to_string(),
            "8ef384af46d89879c81084dfe4fe63b52a205e05f0cb4d4c5c2e6aa2955159e9844bbab3641db8d626188cb4ec05e2a5".to_string(),
            "9097b92ca4c523271b1d83f13668c71475e2fd04b698ab52b377283c9c4cff46f2ca1f7550dd994f7f965612b3c71314".to_string(),
            "898d116d692d30f5d23817028046ef03175d7428fefa1676e4fd2433207237644415be97a5658f827ab3cde442bd85ea".to_string(),
            "a2ecfd4c9da7f7dd96feb7e735b8dfad090e1434f1b46b5e1bb48fa83d289a12fcce4e044fadb79e1af243d03a26eec0".to_string(),
            "904a1afd780417a3323d329c212b6588fedab1131a8119665137700d82e23ce787175cf5e01e38304f8b11ba52333acb".to_string(),
        ],
    }
}

fn get_compressed_verification_key_from_polynomial_evaluations()
-> Groth16CompressedData {
    Groth16CompressedData {
        vk_alpha_1: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
        vk_beta_2: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
        vk_gamma_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        vk_delta_2: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
        IC: vec![
                "86c05c2d9a9efe58b752c969833299126a6eceb11296d56af5482b8306e09fc9264dd9326024cfa475ac519661e488db".to_string(),
                "a172ea8afa79c81fe2296edc7e08eec56b5d3f161436905f84665b58fbe07a63f8491f07301448076dbcb1ad608cbe3b".to_string(),
                "a10bfcf956e0551a25f24fa5e21cbfae736293bd821df9bf95e1c5fa372d964d11462be1837d6291aa0dc386e22b65cd".to_string(),
                "ac53096c6051f18062fd41caabd55e88316a381d8b98e449e572aaf35b56d168dc01efc334a3874f19865096757a62ba".to_string(),
                "a5c567682b669d3485291a5061dd74a69e17ee7002de477b7899e2b488c233d8b9e5d768decb81a865c3552fb9ab4539".to_string(),
                "ae4cd12397a13c11b820958bd6da7e4800922c46d4dd8482a7f4cb6c1c9cff7e34afe9570b49a393eeffc52450168b0b".to_string(),
                "a4894745ca434a03dd693e6ac8dc2de64e47a9e12dc400e12aaef581a268b98c6acf76f5c77bf5bee215b3b9cc2e377e".to_string(),
                "847e4d18fb8fb678abf33cccb508e84838cf587d5e2938c9380fc55e1ec70fc321848ee099ab718da883f53b3250a7ac".to_string(),
                "ad535ba41b608e70a0fc5831ae012719f6571da490da1f8c803ae54499180e10bf08bbd7765c2e3e9508508759942691".to_string(),
                "91de685a25236f65a61bcb41bde98b94e5d4952c9508ab8c89154616b5245468b57c67080e40064617fb762f2a3bcc83".to_string(),
                "b6ce0643657f9726f077cfca9b7ca20d9366d8cd6d1bd7f32c0843a045f3befbaf8adfca6d8257691b842d42d5fc118d".to_string(),
                "850e151e72f15610a3d4eb7c49b36ab348b52adfefb000c1eaaab88ae17f92cabc970de7a8e8947a15ff8ef03cad5a18".to_string(),
                "82a20cc785d17563763fdf9c7e72c1daa1a30bd8f6d901ad5fd481394ff941fcb09bad32b6d48db9cd674bb8cc889071".to_string(),
                "b436990bec753b21804706ba449fb42ff228d9da732cb591ef984c17b9f087776979f777b410eb930097432c5a8fc4d3".to_string(),
                "97aa7fab7a375d55297899a897cc2eb4c8b3b300deaf2d81a094130b59286e8d691c609a41204b5fc24cc8b6aa892c0e".to_string(),
                "8d163b825d42b5eb64d84015c19317a8fec11ae221cbed32ccdeb23231346bb9a78f45f754752a434583eb79c48f26b1".to_string(),
                "a394723412978f6a9e26f8a6b544f7e2ef19e046da6f2a39d2d28f3afb8b7a60c2685ed19f8bd8582c1aeeb5baf12e0b".to_string(),
                "84460d739a9c350fc845a44340956accdcf322f29fe6b8dd67feac92cb086601a5389f7a9ebcdfc3988066174e853b09".to_string(),
                "809536b18969adacc6f93a3ef074fc28813db0943d4cf592f02d32bf6594d5fd6cb6f6e203e96e992d51a8ce63e60da8".to_string(),
                "a003b06c8555742d4cee6d5ec5e861438bb0bc3ba3b172fd48b02a8abad1c0105bf0a761c3567e2d78af37de70f4ce34".to_string(),
                "a00544486182193e4a33644b0a2e69f6a565c13ece18ddafec35cad20ea957963760cc595615ce1041fad4502200a1e9".to_string(),
                "866644b9db709404171f4a4f9cd175d9c0beb075b24aca13293247f7ad39192593844c6127ffe4a0fef85d8b1ec3dd46".to_string(),
                "aad7a3cedf94f2f8bf832a55b94afc7b55239bcec04b167e35fdec726ef3ad48b682b22e73a20d6affbcb5d95be1fec6".to_string(),
                "a16c593325f4ed2e5ea8444ed049f62472e7cb08940eeba775ce270b3f0518aa93f1b339b5cbe2eb37fe4972d270af47".to_string(),
                "9531f25846c387ba315a64f4070981884b6e46b9a9aeaf12c1e1cb5cf2f376b73e01f23f9f33a800bf341acef468eb3c".to_string(),
                "926dc55cc94427be89d346eef4db4b1cdd7e36f7fd8a23af72b462d0af452f5832ee60ee8ee63988846b996e3f441910".to_string(),
        ],
    }
}

fn addition_custom_circom_vk_compressed() -> Groth16CompressedData {
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
