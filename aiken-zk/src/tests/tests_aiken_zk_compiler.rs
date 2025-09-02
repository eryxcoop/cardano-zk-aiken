use crate::compiler::aiken_zk_compiler::{AikenZkCompiler, Groth16CompressedData};
use crate::tests::aiken_program_factory::{
    aiken_template_with_body_and_verify_definition, verify_declaration,
};
use crate::tests::circom_component_factory::addition_custom_circom_template_and_component;
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
                "935812017534174699423ce87649247cb0d5fd51e44adbe8c1fb7cc877a69b3c68c226f2ce67ab5eaba0edeb3cb7e930".to_string(),
                "87f2a484e170837c68b694968cec15de6b7c7ac4bf08678e4eca0021c80f51eb4c6e99a611c3600f05722fc2d1154fb8".to_string(),
                "a1114ac1687495598432d77d8e184f70023a4dd475179512f6b75931d80a2add640bff9584b9b7d59157695ba246e1a1".to_string(),
                "a3c1defeaa22d68fce7b9aaba2bf7715e32253d3eeccef75dac1890cc834ef8e1ede4de9f34c970644104d0ee8df59ec".to_string(),
                "836a9ff18a756b74a79e85c76cd61d2ad0338f6cb11539ea7c6c578cc1556b53dd47461c0b1001e3e1743f289ba9935e".to_string(),
                "aa3c86e5da01e006eb4c751cc06ee49857aa46cacbbf1ff7a850658a496ba5eb9f7ffbd4644ddd7989b29b6cbc40c373".to_string(),
                "a854b227732eb111b7915a560170a1feacaf14fb244b3183dbceb96e4e385ec2057f5fe4c00fc81ee796f1c40064a9f7".to_string(),
                "93f440dc67dc089c2a77cfcc5a96149be8359da4c4382d167b0e24249fdaa9ecefbd766aa301801761c5bfdba8a23424".to_string(),
                "b487097fc842b4964b3b61b8105726a9550571da78cfb3f06a2cfb04698149c06bafe2db73c5493cc17083d9624e0883".to_string(),
                "b0dac1a57981190aa870710afd703b87e48c9b8a5b08d143597a905c571fabd2533240daea6c56392a0694df024f659f".to_string(),
                "b711fb2e827616381cc51723606090511049c0b6a09a096e46ac2d9a9241620a9f4cc4a8d877d3e0ca3b06f12dfc3c8c".to_string(),
                "b10309e60f5c310025270413c1aa663e5a622d4f42c99795064b2f7087926b9e26063be0c691aa55d0e02cdde2d42271".to_string(),
                "847a44bdb87dd1debabaf2dc4a3445a574e7f880e6f033fe7f1ee82707467d94a5ce29248ffe333a3432ad5db324bcbc".to_string(),
                "90fc21db3e28b491282a2e33c06b22290f8367bee95e363fc293f83f311f653851c335480f7fb8c53c18b890e8f9f830".to_string(),
                "81f5128eacde6dc467f4b47ad7fb094981a84bc4d33112da0d2f5fc73158fe70d64eb8bc0221ccfc60fcd00cd798a6f0".to_string(),
                "a0f5f8a3f302d933b5b0d4d2187d9ba28fae79807aab983b0fa1181be4dc6b8baf91a93dfda3218a35e9ad70baa6b3b8".to_string(),
                "a4949e3659af656a1b11d86c74faf54b0bc6f21f5d3ef0d2142e462eacf17cb0def8f17228ca7902b830596e7a7c59c5".to_string(),
                "aec03c54faa9cac792a683d578b2d8b57a6ab68dcab4a674ab4a5ed17bb66b0d30a79e87a1516039ce2fd0cb48d2bbf2".to_string(),
                "b11b2773b64c597c4b7a25496bb6e7d5ccd01536fa028bb5e9d18c7a8701dacff043e2eec10a46d7afe28f463833c077".to_string(),
                "8bc8d7dced997b6bf84df1b6f358552093ef9e4033ea7b393c07a81d0f47028102cea6a624af2ab71feeabe13baf6f9b".to_string(),
                "ab618e26f8f582649722868df5017af6fe9d6df8f90e9c779ae845b71c45ff225641999fa0c81f222bd721bc8d67cb77".to_string(),
                "90333aefa1a127516c0318c7cf1e54c32c07e0c94e100a24d490629837aaf1c2b80fee833ad425581580cde6feff749a".to_string(),
                "acbcf07fefc3abad01703b55fce34ba1a0efb4943374938f9ffb1c6dcaf03dc7e494174083ada845c8f71a2b1aad1406".to_string(),
                "85b521138393e52d4da9d7fcc66c43c659b0533980e48d0f027a64b4e8d697edeff30d68f0f727a4b27b10cf440a5f40".to_string(),
                "b6df4e86c5628527e1d64155f4a9e245875ebc7ad195fe199ec1ccf52ad0738db8b1e9b5679f24c78ce5f03db212a375".to_string(),
                "83dd46ba90f41b8d032d39b98cebcbe145d877a4c0a8ae5f7947c00fb215405f2e08097e77b490698a568a8b1e1ef8dc".to_string(),
                "8701cc63ca8ce6b6e416c707b5928487b92ef3874009f18e6e6b588696ef3f4513df8cebf2c1e6e57166e37aad91a9f1".to_string(),
                "81b87e2b57e285d9a6517c029476af228a08a9366bef8c71d6fef66f2d6cf5465012c10e00e2e8450a96d80109210ffe".to_string(),
                "8d79d7a9a3c9e357b9b0cf16c4c45a37add4b5df700913ce91f3e0d283fe9b3a54805e711d3a56113da52325c383f703".to_string(),
                "905b673333e734ae6e7443ea0abf5e0f9f7955e2e7edba87b253b1d6fac0770a78a859065b2e0e348265cbc218ff99a9".to_string(),
                "ac1280cd5eadfe6a1230baa9ef9527c9bb984af7d0c2dbe2bc0307dd3a1c64f78913b38c73bef50f1b45496e08d59fd0".to_string(),
                "b26e4f2f39bf83787d989751a867451b56acd2f0103601e5046cd027a84ac418290f951afbe64c0f60c2bebce2d96f28".to_string(),
                "a8a2873a59249450619500d1e36bf629d48a7d3ec2958dd7a3d6c260c44d33b058663349580c24125d13911540a417cf".to_string(),
                "b1c5965f683ae1ab0d081fdb61766d8b049ca1fae23cb237c754ac3f006e6e60a82666eaaa457d90343bf75acd684bd3".to_string(),
                "94e365c9a9d1f4b4b56049b187b4e182530901649e8f25021015732f20d63efa5602ce3e1950bad0d13356e6ba88a424".to_string(),
                "81cddafdbc69edd8474e1371858507b0fb63b6c8285833997f5704fc4fbeaf1d9de1f11f42260ede2ea69004650968af".to_string(),
                "91678e4c2ffdc43ca385d5715e3d12d81c7b00d265b58b87ed1b8eb92bb8c8333c41284dcc179107914db259c2f55129".to_string(),
                "b5d25eec3c711fe15da8518371dc810a345fca153f75058536bbab82dc78baaf6eed03102d4169b7d1911dd32d7bb8d1".to_string(),
                "80415f45978e962818b79e957681fd8e3723235c5639f4714e896beb72216cc557e9e834b487296804129152bd99c836".to_string(),
                "b1971ad959adcec7c5d95cd87f3917442660b14993d59634c6c78950e80bb526ea4fd90318406fa055653af1c38e69fc".to_string(),
                "8de70d4118b3fdcb11b31efa648ded1536b5b76bf9c2104dfa2113c7ecef38ad689d75a78c45780c4436fefe17af77d4".to_string(),
                "a71f809c946c779e8f6853546cf2be7e0b7da79064057fda4695631f13f9128fcb1d3d82a3bab90114436897c57e1b43".to_string(),
                "8f76416e811bf56631ab98356e03c29f14d952ddb08bbc2a25c6a0aa2da33af7c21d9230366d71538b0108e9baf07fb1".to_string(),
                "ae3b73ea483f9c3df010ec9f42f39840f529ca79a4ac143d8d7efda8b3040589e53ec8b0cefa2596ac2b77702578757a".to_string(),
                "b29d325aafcbba553363fd59526032b3ca8605739045d3fc1f47a2e0a07f20fb4c2a0e08fd046cfb61690717943c1814".to_string(),
                "893e1597884844cfd6b976f995596366fd0c184a1167838bb6b5fc1f798e6c57f234c0f341e10fd9109ac5457c9c8d25".to_string(),
                "b24bb41bb4ff91a8a12305bd2a2adf4347ea7aeabde451289fb6074f19cade0df8974ece92618cb56d95c1a3a409beda".to_string(),
                "ab2cdfc53653c93a8bdd3bbe80455d4e7ae74ee54f007a5dc2603698f256fa558d0858d3e6820a72165053cedf64a832".to_string(),
                "a4904037d99659bd63461ca6750cc703fd76cd4d665584853bb5cc2badb9f598f079ecd26a233f2e8532a87dc0d91c69".to_string(),
                "9364f38900fff450901a16402e0a20b1e72c168d76ac7874612bb2a6038e27f31d30b6caa5361b99d85c743cc9bf3aa3".to_string(),
                "8d66b1981e40befe23b10a81fadd40f78ed94f640b9c2beb5ffba268816cc70425074357223f43fec7248c3536681c4d".to_string(),
                "a259354676722e131332a02b17a2039984ab5260ddc95f25664ea7371d2a8a3bfaab4eb6ba6a2ef88b51ac195d6e639c".to_string(),
                "8e05bdf1e14332a941b7f79c40479ba11cd33a23fe09415023e4ab57fc3ba0806724ea2ca1cd0c31295c449447e19d66".to_string(),
                "9378eb132ba3f55b96772de449a1c6f5c6a065e261e0966ff83315c57224f55edc599bb077df462e104b6240bcdacaac".to_string(),
                "ae204c4fc7712cdd8d73f446bda85c2987419e2bf51c857baacd5a5cfc9c3d07dfe590a272033d4b58ac4d304e0fe68a".to_string(),
                "b9ae643a2dbce5bd40d0c717433e09fc69f03195665fd3831251814ff6fb8be5309ad8cd461037f7a285d010af30e049".to_string(),
                "b3077684b74d1ce04c67fcd59be53349bd94b3141700e9be565e87e4f60bf5473d45cd0526cba943a64a4fbcb00428e9".to_string(),
                "81c5f0cce3ca88553d881cc19e857392f6be423008d2b2fc750508a241acdbd22bbd8dd150b4d13b28a4c8a1bb6ac689".to_string(),
                "ac0e64c1bcf8de942a73f1008bf6ecb6f3735a289fff9595d9f4fa324bdf9edb71acaf238a3ad83bebfe4a933d87195b".to_string(),
                "95d7f513a2ccecd5166b64bd37d2a5926d01ed15b1102fa57c813eb9fde6d250c194f3b09131d225653714e0eb254313".to_string(),
                "8b60bd2635c17963e2c40c46788dd17708ce0e2c7eada8024ca47cc6fdf961e21323f88ac3e6d464680b73de20f687c8".to_string(),
                "898bb79b4fc95cdf7cced6563673a3f23927cc8bd9a94a3456ae3ca52003160b1b17dfdce3bc2c6f97fb316b04d81e78".to_string(),
                "b76f184814b0715663a45df1e463e9c1881148b7cbec33907d8c2eb3dfb1a1a917c572218c94787059e3e7f555b6e3d2".to_string(),
                "b31567b124fc72efdf151f8e2d0d7d296b0ff23780e6f84e53e72cb6d857faedcc736c7e11f57bf0fe923cb9df5e02b8".to_string(),
                "8d159b0359b1590a2c211555a93767379946299e1a31ada3988c8875f425bcc264035e9b7dacc25fa1e88a635476400e".to_string(),
                "8290ee4e091cb9e4ad02edfa9119d38fd1b52e0d54ab031f6e1bfcf6c919a6e2df533d4bdad9e4dbc9966344b6b1a14c".to_string(),
                "b6c487a427754fd03626400eb6b395cefbc7ccf934c3c9826e1ed1fe0e90a3b1f7ede6db837f2df34630bdc321220d8d".to_string(),
                "8fa9ef5eabd9d108f1bcb511d23e36103f0f83d7802b7c820a97a227d611be814fe3072c476ded6f280b6c53412a4b97".to_string(),
                "a1248a0f040880e86d9e5714c8170b7ecf1931e539d4b694793e3f89a21ee87b4b280b3e09c7b8647efff1924f0beac1".to_string(),
                "85c3d14a2170a12af08cee4ca81a959010244ba271e6b84dcb816c63281fea56834a8cd832f1f9503044c9d7a0bd7b7e".to_string(),
                "b6c319ca80ec0694298db6c5a67afd644b85d7af572f3f52a0ba499509c29e970b342c92a915e74183b0eca18e2eb95c".to_string(),
                "b873e82ee38982781e0350d8b0785b42ce8e3059e393f1b29a446305a7df67f3dcf23bed778418ea415f810dc12690b9".to_string(),
                "8da9b88c48170b959103f66daecfe2ca4b6ac51f1e7ef05d94f939f19c66fec10fe66ff5e76864ff769f055e8943ae9c".to_string(),
                "a9de23dfec0ce82501878f5443a04d0fd0da60c3e0f1ecf6ed042a7e7518d72546ced62dccb3bf67ad7a2f6e615527d0".to_string(),
                "b60cbe47a7f97ba1701c3dbbfd5870764a3dfa664b3922f5e696bcc9e28e18b61335736e3eae4b7a0310a7b18e93d4b7".to_string(),
                "94b95b4d421a89cc6db28399ccce2b797528812293a7a0f3c425cac2ec6101b3a11671f5faf7d06032936bf66bdcb47e".to_string(),
                "aac90beeefd499a106d9805c38935f579419e78a5696c2fcf580c479228c55c9c9a720e2b364c8468edc39f8579d406b".to_string(),
                "ac3ab228258b6e4ab4f5610a4e25e0fea06ace45321e3f06b39552189eac7685ae58c46c3e5d45f471030a15c537c0f3".to_string(),
                "8e2bcaabe697969150d0777232300ad38ea2fee36e18f6c941713a0c405dc029a5420288ed57d341e695d445a731e2da".to_string(),
                "aa453f55908ae4410e9377170181302719b5ee1914811e5a8c57c310dead4a5b581187eb6d4e0a3540c4487f4c3e6d61".to_string(),
                "8d2edf90ccdedf285efc7d8b382c68b1fb7f12fe90a75c593f16cb9655cfb1f55fcb4b2486488538745c45149328ab62".to_string(),
                "b0ac9e8fbe2bc38ded4d39d2b0cb970636fdab3cd31bdcdc5a12a6af90e316936790c225c5c19d0f6bf4347b11ad7921".to_string(),
                "b1b565219ac55c04e9f8caf1451fe48807ad2b303b0cc14be332ec756566810688f709ef602ef3347f81829ec5db1513".to_string(),
                "9091de2de42d818d33a9ad204bb87817116b2040ee5d1c3f94135f01103dcf2c97c3b53937ddfa7c385f6aa46d87abe8".to_string(),
                "936c82cc1f2ec816daf3abfcad017a11246837508fb7c56bf58bea6dd25e8b30488500387afff9c2125e493137ec96b5".to_string(),
                "b3283a4155515869ee30fe69d4bb974dddcdd854bae25e44be5a145754ad2a252faf64056fdf256ba89c186b7b842fed".to_string(),
                "9065df51c9cbe87bd52a39ab474253cf3c019a5df977898c308c312cf34eb70ea950cced495bb3e80876b39f9a41b9a1".to_string(),
                "83c13c0ee96dd4e4448fdd8530ee58e13412c85275881ffe6ab1a373f02ea6dd9855703173b9222203aba3cb621b878e".to_string(),
                "99f514fd96111e4d48c739914245aa39791d7678cf09b184f2e34712164909152979d1e5c845f049a3b682cca9657031".to_string(),
                "ab0375593065c508538fe7051f6957cb8b84082c2f4e963a7c2fafcef6451e03c880d8fd787bcc126ff983bdde153eef".to_string(),
                "a93b1532e209aceda1fc6bceb090a4654b43cc772ae4f6d7a72cc8d82e056d16bfd113deeec0610588f4b11b7a9bbadb".to_string(),
                "86e3d9e17a8e66dd38aeeba6e1d8add58b5fb2d519dee3e12375afcf9f89be8115ba87e117737ece6f81395701faca50".to_string(),
                "a367c55b24f0ffd5da27e01d01a9cd401aa9d3db5441efb023d81694a6756274e59f1a9e8d04345e1c7141f588bbdd80".to_string(),
                "a87fccdd0fcb7f3edf075795dc88b1d8ac97eee9afdb62004fd7c5e1a82003eb24118dc1819e56ccebcf6035976309fc".to_string(),
                "90a5e946a85912a39a2bd2a2a65239167fc105c779682d1023042b3c093e6b275e172605333a255c29d18bc5472283c0".to_string(),
                "aa02503ad548527e57e8cea3fe2da3e1b5f5e1d4b1c2ad82a6ba5cd17f6e5e5524c938091c13b77911c33ae5d66669b8".to_string(),
                "904147094da1602467a443e6bd5d91ecede50b4b1c9cba6e6a3e022b5566e1f2d50349fc198a1ff6dd8a37d19428277a".to_string(),
                "89f786a46c8906f09ba92b2d86727961371909b0edbc65acdf6ed463dbcf2ea117f455046cf452df870cf1253fc33223".to_string(),
                "a5bbad83f240abf5d05f38c656125f0f11338f653f824734419469832ad6b953384294e30aa73d3a155ea8eccdb230e6".to_string(),
                "98f422d8aada2f17c8126a8390b97527a5689775726ce800e8f17eaceecd83936d022b180e1bcf3149b7dc857eb70340".to_string(),
                "a60020f218eec549969e7048605d5ea1bbf8394d2286ff2d0c0d9037b1e25b1f21f291837d4c3693fa9cc0cb54f08628".to_string(),
                "b5367e45cfd6432eac95f8c68a0290907609ac0d6c688d4a94991fb71d329f286b50b0d41e2543743a64aca2f6ad2696".to_string(),
                "acbd38b268ffdb82bdec67dbb5295f9ed5212dc6ccbbbeca3a72d724ce847b2c7dfb41c77eed99d65e66101d96a916a2".to_string(),
                "85f6fd5afc1a93281efe4ed8af4be96646c97525a6001c141083d301147f58c78d6dfb8d9db1b3cda75cf5feee37a750".to_string(),
                "8d94d11e1bf5580726f75c882716a8dc9d2f74833dcd27f2bdd86c41422bda73175e854ef1352a35b2ef71cbfdb34e26".to_string(),
                "a58ef8596920d83d6e73378922d1cc6d186c28f95bfc73bc70175144e0dd8c80d822e3820adc5fdf3cb3bade87cf105b".to_string(),
                "999475addc6853c8b7372d921e2bf15e0bf8ad9913ef16579ba1ed6b56ab7acda9eeccce6a45c75dbcc6fd646b1d1f2f".to_string(),
                "b0572c9c06c0fbcdd185b7caed446cb9e6d5b8c37bef22035d47eb8dff07035f78902e674b9f209f9b74b95db229251c".to_string(),
                "a9bcf77679a1d1b0036522537a7408098840e1869765186fa4fba4c5ccda85cbc480fd1d1df9f1c5e2af28e8e4da6341".to_string(),
                "864855a36c884f7bca5ed3055fe57ff5ed47c502b167a0ca343aeed9d9fa89599cbaad19a79c55266e4a27a305bc9770".to_string(),
                "b223c6cc9acaffd94291833535d01c47d82932f5c274e97e99253400a248c356955fe096b8e731db333f8df0d251271b".to_string(),
                "8749a2a5eea5b42e685e107f543de3a9c0f2ce89300b36a88ce16702c22f38b8ea9855fd45456b73897ca831fcb531f7".to_string(),
                "b704526f2a2b743fe9d76f7a12b9f2f0ada50f793ea8a7157a7214439b990ba0183de3ba345a4d3f4fdb1440eaa2a220".to_string(),
                "ada5ddf9d13da90d11bc0a0bfd0d326a7258e39525055172f125827045edf9cb8e3c81887b2ddc38befc25b341e3f81e".to_string(),
                "8bf81a2a1ee5980bf1bc8245d7ff47da14130d0d18e3d1014f3be187155e2ba11ea5e5a22dddb006aec38a1423a6fd7a".to_string(),
                "95a0630e4d6d8ae6d2ed5e57c546fb48d96921fb645a4f3ba868b11029c68d5a202dff0c899da844b1051350768ac4b8".to_string(),
                "8113624b8d5a93b7aa3a210cdadfb0a1cf239358c4374c8055cfab3bf1a23be05ab657d2b713fa177323ddef9bcd5d89".to_string(),
                "a2efc25e572bb36acf85e114298367e5c82c57944cb9ebfa8b9cc49a10e214eaafe0585eea37baf43be96cf756aa4288".to_string(),
                "a9787c898bd8a82f77d5a2f0f039edde0a9a109aa9e4d02f3d07f1451e633ce049d75d59397aff024287890d6dd3e8d9".to_string(),
                "8c73b5518fdb64236feca44c23a75c542f9e08444ab1b9fef8540e6ceedb960fbbaafd1388d7de2f5097017469c9a9ec".to_string(),
                "b9bac9e98ea63c4de8520dbcb342b456a1c66f448ad0de56291f9ea7e6fe56651cd6b4998d27af7a60b43a8a5853a2ff".to_string(),
                "81407dab27bc803faba3bcfddc37c73f0064155a37e8d6f82e18e288f87635479a6770fd74bd3fa9465de96b373c905e".to_string(),
                "b40c7b6fb41c71f300533c526902b537bae1d467f2d3dec37b8bc8fbe2aec0348c9093dd8de66b78848c80b15c5069b6".to_string(),
                "a2e55e1de2ced39cfc641f0339e5ac183a3e3b90ce63d3748c5cc74bea3b87d2df523dcaa2a376861037c6c83aa6e9c1".to_string(),
                "b556af986ce192d3fd17abcf6f78ad88c13c041064f5d0bb9fb5525e7ea1f28fa8636cc3a3bfd2a5a249e896329ab48c".to_string(),
                "813d99da4398e08c7d54fe9f6207e2ed10853c2dae395ecaffc0b62a5f76547c1ff12c7771e7391c25247e08afa0c647".to_string(),
                "b876f824e82ed624c8458b31e6ccccf40abe76a56232e56527f01f66c25621351e1a6e9cd8b78ad4a655caa5ea082fd6".to_string(),
                "a269631237d91eedad05131099076208c0f5746ec7b3e64c7f6b08a494825044b8b67b79d36e1d87625bf1e7014c016e".to_string(),
                "a25adc058bf2d29a364a084c159fac4b47dd2261d88f8ef0b54d35726a3f19d37d6ad415798e0e505839a903b49cd6eb".to_string(),
                "91b94551cababa8e27e0ac3e099716b6fe74088997f067adba210f54f4f2af904e0393ba0802612dbd56448ac1801236".to_string(),
                "98fe1b04bc02068354c1287b63c1841c0bc212133478dba3d7267c9f2015440f1ae43fa3ed6fdfb333b097d76b1b5f34".to_string(),
                "8f8a3da300ed1e311db94f29758cff27370dafaadc7a853dc4471e4303f77fb37206d72f839f1488e3c14617ce13784f".to_string(),
                "84f3fcd2e9d1c7f4d1ef2b83d050c923b862e63f7f61f0b26a46f2608c747a24e3ceb85741a7483868f60452de20b433".to_string(),
                "b9d96e6806be9177385a0d3f7358f8ef45ae907917383f6f9dd863c0f5203856dedcb643dff163fd82fe0b87cb090be0".to_string(),
                "861759936089dd141c3e65e58449511053b7fd138d8d1487d7654153ea9f72b23aec5d4ddf2d1aa4d439cbe319e02688".to_string(),
                "8e3c93930d87b6ca2da92af2611a1eac1aa01cce724e56ab90c38f0a0b0753e867c4b52c50ec1df141453af8a230512e".to_string(),
                "b6dcd73a72d2848a126e5a26c2143108c0cd4a2edde0bd6f360662349d67e0f54446371bb681262886d56246cb6d3b96".to_string(),
                "aae8ad02a78db43e41902df20aa542b43e457c8d9ada084991c23901d1bd7e70b02f7886401839897ba6cbfdc1463b66".to_string(),
                "abd1af1c28d67bb168ca4bc9e684dd5ac6deffcf4f609d5437793f20a811407af34da19e821873e35a737b6d1d236093".to_string(),
                "b65a0b5b59a1f2e6354c815a83a47bb0a32aa0fdbb6fb2fd6d1656a855b8f2db396c59c6d65c73cd33f532480c014bf1".to_string(),
                "a3ab9bd7cc6c51ce7c37c93b79300a5850f0e6510352932cad60315ee63ea30080cf1d87417960c02d9485957f0517d2".to_string(),
                "a6b75206ab269766ac390a3adfa6a46308942497613ddd4f2947c5df4c3408055a952362bcd29a9f5f52ecea180b5974".to_string(),
                "990b7c0fbb0265c72dd3a30bf5eb7b991d20a9140714387dc48a8aff6a136c5a695cf2d493475d47bb0dd1bfdfe516eb".to_string(),
                "ae741b200d960f715a6b80d9d40a8df0c5d89afc8a86a6fad823806f5210fdb343722e73f4dc92c4cd4cd4f22cdbbe68".to_string(),
                "8ba38e639c83bf99773ba759b105548c53b9424a8d7c5e43ca0cf1dcb625177f53e248a0bfc8178630ad3b0c95788c55".to_string(),
                "a29b7c18ddd17882cf3babe9c0db2fcff2f2f4904b60e5c25c6a78cbe63a75aab1f914cb05bb616e42f8b6202cd702e4".to_string(),
                "92a805790e0ad8c790178d2471fd696bf78d384b2396eda65895ce4a1ed5148d4e3330a0079cd394327dc66acf6121a6".to_string(),
                "8d79aa2c186d0f6b241c684e792c71a002442dcda23689f1708a448f60e92b333276d064d169e0749679b79e3949e227".to_string(),
                "83f445e5ded5a81114e4841c615f1e831d90b97400278869a657a967a3c568327cf2d12dba373be07088cc37c0291840".to_string(),
                "85c92b5a7c806fd4903db690e9cfa0b91dc58b66ad0e6823fc077719a0589a9e91ff404b7c752b24d1242dcf42253dc1".to_string(),
                "936a2b8fed3f4806c7766dc23c2f3af475ae1033a37a269765f3ddc794a6da13eb2a4b02d08e65afb2dff425483127a5".to_string(),
                "b97c7b2af82bdb4f7f92f8aa29eb2f230297a43bde3e5e1d9d9214c14e497bd82e55f012ba2ef973508264762db68299".to_string(),
                "99c98af67e7cc2fb0b4b5e5d99a846963addc3d3df5b1a099aed3247a737148216a2c1c633b857190c190146d06722e7".to_string(),
                "814bc27a29642cb3eb3987a94b2e558ca2b2b7d28d957368db6531dfa7f63297cc2ca4b34453384fccd5db8b368daa13".to_string(),
                "833cf0ced4b6475c4fce43b2389c52952e8e4bc299891e38928966eb79889734b866769376245d4046141b9c5c4b7e9c".to_string(),
                "8085420932e45bb996042ba0d882f66b439b7024822a09ed7f8c41f064d0506daf6845a8cb1d07e16df81e1432f4873a".to_string(),
                "8aa1b86df3c0821ed7ea3d3e2071f4c69fb586eb6083fa7fc8d8e89c8dcfdf57cd1fc17286457db9f60e6c619df81679".to_string(),
                "94a79c69b1b211c90b6fd46b82b1249560ba1adc7422b8c5e29be056fc5028f31c313f50cf12f09016603d0154866dd9".to_string(),
                "90cfac469e8b11057e0727d31c9dcd5a87c34411604f5c38af241338882bd966c5af79a9b8f1f141508361c881df12cf".to_string(),
                "a60ee41f93fde28d9c19468c5d93f94c88ce9c1875b010622dbef68027e9331cd8817b43783c032dd2072c5af661bab4".to_string(),
                "84d66f88e7f52b7f676655ac2b19f1690fbf6a18f150c6948633465000c21cd9b393e10ab23c57c386832de861fddee8".to_string(),
                "a437216908e229cf4f1153828984fe8878cabc09fd952da5a8be7e2b194248133fdb2fdebc28231c7fb243ac50886a71".to_string(),
                "b2d687beb39103692ebdcf18e4b4b00949160b0157e016e21f44ba66c9afd592bd4f980359b4f9efc78fe752935f73f3".to_string(),
                "abd363132feeeeeee25e907e664964cc017042a8f4e020460f7a5868ce47b68805c2dbaa611c8234462202140366fe98".to_string(),
                "a95bcb7b54631e1439a009ae12a6675f8b2159ebde257900382b075298459ce359972b6bd112303df1fb5a9740da0da9".to_string(),
                "a016d30dbf44aafa6973eec05ae5f26b9c13cc6468e3e958336bb822b9a732679c092605b642cbb95c437a7fdad506bf".to_string(),
                "a035216928fdae5b720cc27507151fc77a0ed5e4ca76ea75e0470c430585d8c4e600dc1a994cc0ce143f4ce9e47961c4".to_string(),
                "8ee364778def402eaefc4da73bf4e49636fb06170032f7fd99f600b5ffe6bc59e27db56d47bfe69c76b3c0d94a6afdf7".to_string(),
                "913238318ac09e6e70c0414e5fd555458cd893074c52a22984ce0e068f0c59c31c1fafc3992e542881c791029742ba4d".to_string(),
                "975b9bd25f3ced0bfddfb160f43fcc58dc078058dc996d03f47180690cfc98b4df6bffde5aae77fd19445a7959a4ee7b".to_string(),
                "978ef23c3cfe31fd512c0dc29f3feb6c026bf1f412f855e3c13b0214bfa89011a7cd21536995c0080b8255df31a2fc2d".to_string(),
                "862e139c7aa0e20faa5b72e03e43820aa02639ff773161d6ecd8565e29e28f6d9207f423ceda220394b1cc4ec7ddd6bb".to_string(),
                "95edd6db121276909648a4c5ea61937d39beac01c5e27005648a11f347c1e5d8f4ae9bceb281b7ed69f11ef6d870964f".to_string(),
                "a452ea5614801d0670ea60f9e78a722f54e5d16b58f9f98cd442dd85ba1dad6d8548fdf86cafff961ba411ed0ee42b9d".to_string(),
                "84f3ad03d517ae77a4e4f0b68a2878614730461faf7dbae13560e80a12b6bb692f8eacfc32f21b5e556e804f335e2e9d".to_string(),
                "9324cbf426714f8fcac234be9a16a74ac00ca2b8d97f967ca1a4cc52966e92675f3efc8256e36a65bb778d48941c720d".to_string(),
                "b86965b7d363161bc825fc37e2dd801aa18ef8aee160eb483ff597826cd7b1d4ad5ebc307b2fa28243fed87e8845eb51".to_string(),
                "84833058859afe496cde4d5db033495af3d98a792e17563bf047aad4095089702f8c0650b27c11313c5577710de76498".to_string(),
                "8d4c689b5e0de99eb675f60c6f26a64eacae030c7c018e7faa9f1fafc69cacfd3bc2ee8fcc78483be0d99f501d964815".to_string(),
                "b105e8fceae4cc12bd77adb338ab519f5fc798eba54d3345181ca32184621e4a0b70942b1a8aa423eb55cdb3d0cc8d1c".to_string(),
                "a2a0b04161be8f0411992a3e0baf3796eab9aa24991ab71072997098c27f92c175186843a51f34caae9d7d3679c0e07b".to_string(),
                "b273b520f6cda295c7f6d5f48560814c65d93f938a6489f5bf0117055a59a1044a1ed00b16b6d1fb495615e09b6f87cf".to_string(),
                "82e29307905e79ed937137c77490911722aaebb979663a8093aa39cf7bbc88d74e64858024503ac5ed5d54dd93ced39b".to_string(),
                "ab1483fe1c1c6ed8da42aa05effdfdc2328ba20a34a99e2972cc0947f3930d2d5094b5313d25c44aa9e13930d3e89d3d".to_string(),
                "b1400419d27b53df5098b3a70e8f66e3febb1499cd8a6beb303993ed13894b59e30ccd9c52d5f41f2d553506d45a8bb8".to_string(),
                "8449b515b61f8d228f502f0191cb8af3939de11aba6845b63fad212b71931821e0de38218cf3109def5f654682e5ec51".to_string(),
                "84429dabe200cdaf0f731940aace9ab636f12f478b415bbe3d094f3493f777bbc264e078f4e09a3b50bc97e1cf3165ec".to_string(),
                "91a9bce72fdae7d552a16792e9603a5de5f4330273d57f53b21aa1ec5a382e1e3e07d8d024d0d80aa6b32361ddffa086".to_string(),
                "85940d0f343a60aeb921ff026d0067045dabdea8357510b01c3903b02db4619940d24fb8334e769b81cd934df9edaed1".to_string(),
                "914005a5b5783f17694080e5be267a4dec6194037312657081651c65e8614efe500389b00d75f6ff43e173b7b6532fb3".to_string(),
                "b849596ff166aba1823a303ef28df16867088df91e386a1090a3f1b6dc7729984dc7d5de7f487660ae2956b22aaf1180".to_string(),
                "b6d2fef4b4be18d2c6fc62485cb79ac8f29a03d98443672ed0b29b41ad51863a19bebe7ea07d7913f24696620e42b825".to_string(),
                "929739a928dd13f8fb01479e050a1f97df774b6e5f03be1df49dd84b8fa3c7eee9abbb177b10ce1c332eb8cfa5a89702".to_string(),
                "af5d7cf2717d242dfd488fdf6b3e8b4545dc4566a243a56ee4f762b0cf08f00322694b21ccf34fe7f35c0975ff5d9452".to_string(),
                "936d119a53e45b60baaf4f1f24b634d2f8a31a168fd562b8ecdb6505f9a28052720b9335d6c64791d3a4715ba87c84b6".to_string(),
                "ad8baeaa2d9cf1e68abdf0ec58ccbc097ab3521505c6c56a69d83c0b88d7bd81e4ea619e7ab756290081e2f7e5686003".to_string(),
                "9654c5822170885789cb714e8da5e36eb278e4083fdca54bf9f10dabbbe98e03052721b60ef1fb45c285338956a0903f".to_string(),
                "ac259d9aac5dfe48ecc434c543f2198d98f53fcf88d1dede5ee7fe334b4475967a6eb70820ec66075ab656fb539277a5".to_string(),
                "ab5d553f621a2d98abfab82ca4821a395c2122d4e18d510d0e6509ed8e1c67f0401b0b877d60d1ae374a09ae90376f8f".to_string(),
                "b87f2d8f2e4ab130401213d5dc1c1d29f556b6725e41cee47afd94b047456a2efbf0cd90d1e1042a8dc89ebc76c59c2a".to_string(),
                "a9275ae23ae5edfd3dcbdf673d93812a0517461f0862150091fdd6d4a5fdd207069164af0c2d60315fb542ef243bba17".to_string(),
                "a8945d97e97c3a63131fbf1609809ba458bb23ec5bfb1170dd68bda700b6c728cf86f6f25638a024d1b0a06456017be8".to_string(),
                "b2b3eff87506776bc54726751d855d6a459d006f0e7bfef1425d90362a55ff44c997e9d1af80a73aafdf79ab26a1a1e4".to_string(),
                "8d19688b06df5a52bf6363be669d0ec21a310c2077605cf6a79792f094f046d958a15664087ef5f366a40bd3f7e20f11".to_string(),
                "9061ca6950fd2af63d5a09e043dc18e756542be3006724cc3c996ffe7c67dd945ccad8dbe0f37d799b31923654154304".to_string(),
                "94843ff0d438d141203d1d801cdac0e9f706e87dc9c49f8ee7ff72546cfc8c10e00aa8b5162a4d64a81e9d2f7da06446".to_string(),
                "b29304ebbc6c850fada9a7f0a936594deed1bf43174aa2034bc19d48360fdac4caf38936331be919f30d73c0ad47a78e".to_string(),
                "a377ce5cdddfa857b103d932947d4159a94ff76311567ff97e63c3ff8e9dd4f28b5653583040d36f07e52436fab4d1fe".to_string(),
                "824284442c994940981605b081aec4a4d31f8079d629e6547c6226dd281c51f914d0f5dd41177d5f13a39212588f677a".to_string(),
                "945b8e8634b9aaa80bb5cd6abbd76d0d389d89c06aafa65e54c306713bb7ba6512a789f984b324ad13354d115cd4c2c5".to_string(),
                "a5f09ab8385c489c4e1a2870bcf08d9c81a1df90d50d4746dcfcd13d192a5d783274eb9e0e0c3b40559ac5dd691aff5f".to_string(),
                "a2ecdb0fdd68111d66fca8e00a8df1f9821f32fcd25e1c30c4f8e0a457f9aafc9e44df78c81a64bb35e44097e24a35d4".to_string(),
                "8a90e1e82536a6ecdb7a87b84ea2e2b08081203ea1374475125e4c9127b76c27dfd5cf855985072521dcdd2990b0a9fc".to_string(),
                "8120c55566b87afd3ee58d14fd74f57ab207ed0e4031d63c1bcec91d50daabd1fe39193ff96f09a3e0a99bb2f53e6670".to_string(),
                "82f689be94eb3ccd6b7fc6c153115ea8ea253872bc09a1d328ab31d7f85e069d6770ce7b4cc2f48c142492b0d2f83f33".to_string(),
                "aea327db456c12ceb12b83e9775b6effa46e8e56dbccce9d34955a93882800e116830ec34c2d18b265dbe3db72fae052".to_string(),
                "90f466bb0a9cc75e2383c0c9b0ef617c74febc2bac4f0dd22a365293701a53ab7df310303605035c28284d247a605c50".to_string(),
                "b21ccb22a09cfeb04d0c3aeddd1f2fb14b11a11a235691562b4ef0a3032b1e361053483831b160ba1e4384d21b2cf91c".to_string(),
                "b7933708d6c8ac155df4d1aeb9aac239cd7f168670c9756b90380f42e290a9df27ea3f1108ecd6964ed34bd9653f4cad".to_string(),
                "8acaa0399f2f1a1cfde581fa58191e055e69fbe0d35722f368d039c60dde292c04e035d8fabeb8b784d7118a61717a72".to_string(),
                "8e40a136ac2bfa4e56e47f8db16f878427d2a2cca68175f638b94cd94177f7bc782d90138b51607b2e877a7995753406".to_string(),
                "a66312f121b5f8ebea7af26adb86cf8dad830d05a912dd2267ec5e1fdbcfc3c97898e57a8e6af1c44915b0245ef7947d".to_string(),
                "b0f7da9ead5096fa88a983fb0ae5f73a5ff9d8023326d61924e4d37851156a0e874687eebfac1bf539af874da55b9fcf".to_string(),
                "b2f19ec368236c8bf523befa5378974007e77666461bbc94a3c8f34e11ff431ac1bef24540f774610b15fbe4b2243bd3".to_string(),
                "84515b2856744ba8b2b22914a391d69aed5a983a4081650e6ebf854b1578406bec3d2194f9fb9faf5ec2529c0ad8a761".to_string(),
                "aa71eb14bb557b782625110330d27f0b69833eb98525881e5b9b52e34df45d3a452d26206c4caef165f055be5e5cd55a".to_string(),
                "ab8aa49eff33b8817997213bb38e9c9d6d72a0d2e03b47f692781bc0d1ad3bfed2b4a5d38965e8bad8f28882d9c5b6ba".to_string(),
                "aa76896efec96cce81bf757848647ff8c57063464aaa44fb82cf742596af2f1ed687f19bdfd058a68bec590b28b6ca09".to_string(),
                "b242afefba7b96d450bdbf2d61ff47c1c80830b0e04803189c0dff72ed52430095ba237ab0177056c90e6317731bb4df".to_string(),
                "909650c4e81fafd05c427eac76180d263a6254d61fbc222f4b105e2f29e3b15a6217e2ac619567e3bd3dd10646c2a63b".to_string(),
                "a21bd455a29571e7dccb1e4ff33a621b0b583ed07ef346d977b92e16870d25e297f22c85243e5d6a560c69cb53587434".to_string(),
                "844d0b9491901a74906d535cf446da8b42bf9ea18309d1ba4ae403e5dbf047c84f865a23e33476b74d1cf5744d041477".to_string(),
                "94dc26a602b55869f1915d47c7fccba2a341dc056d7ab4958321a0f62aaefa322fcc51c826942e9c80e34639a7039dd6".to_string(),
                "8d62813cc1c9cb10b21a0f7b2f848b308e2f53f9c22183635da7b19717e27eba1d250d3de4d802eb150ab5716316ff73".to_string(),
                "908dd450295649ac80de8202f3c1957596027cb3922892bd3ea713adfd52c0805969b7a0e1961228f4f9295e3ac93bb5".to_string(),
                "b78599aa4ad7ed58679aacfa66696303166ea18e991c95f94ba16c6264905df88ac9c8d803760ba63ca97156bf11e1f5".to_string(),
                "a25c195fa5bf2e5f9b828644e25801fbae053c2c9f9c695c56f7b2b4a691ef7300dc58dbfdf0da418ad586b4003e72a9".to_string(),
                "92fe378c6c8bddc6b23601716e2941b3326e1bb2776285a395fa2db7f41491bbc78517eb09a3fce9b1ac164ddb56d9f3".to_string(),
                "91841281754cf4690becf4026b0547be145bd826e9b0edde757f08cfeff38b7bbd5376458f432114cdbe16ca9a1b0818".to_string(),
                "a9e8617c43bc92dad3a86bb21a7b03ca4f930aa5293316f37b906b8cc6b8d9ee2b67f61c7c32dffa2c782df074860716".to_string(),
                "8f27cb077fb7f03ac58b6f2be24769f75f804065d01bc0af26efc1b878b3c2ff984c38f32f9279bb69ecfc3ebb26cf03".to_string(),
                "8c253ff72af6dffdc3a851fe5a16a43636a568bbc1259dc8c1222b60a2fd50c76615d31f4dc7dd38d3c502ac145a82ee".to_string(),
                "879643e658f6b3f10891aaf1f5b2b525bb57f8923af9585c7359fc1ca98f62b9c03834e67399cc5b6fd18d9c01721374".to_string(),
                "a7c41a983554dc3f58e12fa59501c66224421df267aac5ec515fe6f0938bdfb7854d8b73e8f30641f2a0caf07f0441ce".to_string(),
                "b491c48d1dd1743df0124a656c026e67a57de2b447d91ea4feb7ab383d642f46a40e5bf4bc354b6f9487c922a3dfd8a8".to_string(),
                "a4d210d0a3f13de560ccb193ce8937fa8f8f9c710d33d3f3259730dfd5bc22227ae2dd61c83eb0b7dc7821a12697cdc3".to_string(),
                "b95529859f14ff5b4ebd7ba96edb73c73532941c141df3e54a79fe80beeeef2ee1d9e23c27356f43f13114a4f3d6bc2a".to_string(),
                "a4f50fbbfa5a0171aa199738080b4d42619f5d6085714d7246d36f484b8401b4e106c6508a7d056c5e58f3c6eb317124".to_string(),
                "a7036e123cd074dc107990af16684480cd19a502fec64da79bfc5076978fd75ccc10f25ba0d78fbc6d7870f1d7c5da64".to_string(),
                "b5e0efbf90f526e231e77c336eafb33ce42317aaadafa360cec5ebfc68add27f3d52451b63f03212084131740c826797".to_string(),
                "b2683ed1b34af36c780bad68113c16c337d766cede2d6dab85b0fe2d97557031faa4cbea5db2072cab361747b1852129".to_string(),
                "affb73a5039ccfef7ca5be4b15501bcb5dc9eb2a7a64f8ea8ae5dfc640b8762775d6e89eedeeff0ab70406edf735ae19".to_string(),
                "9289ca77ba32a28748ae1b4db1415a5cd0d6612bdb2beb613eed440bd9e6fe6038caca4f9fda83214c8be5f75ae3406c".to_string(),
                "973f42e88c18436bcb3ace99ce9409a27366759a0f905326cb873b4a9ba7114852812dbc2100c60d544b1779e38df181".to_string(),
                "b87bb4a3f80c3a355cfaf584c380b9ea4f2f7aa509f77f2840b2e715e428cde38d30a549d7960db05bd0dd4dda7d9e65".to_string(),
                "a40b5731d45f2a49d3b0b6cba56ed6e63ad06f0c349a88e2877b0bb4ca1ffb116cc3b638a1751f4fdee03b6f53958eff".to_string(),
                "92fc88d23f23ed88f4f6f029aedfd06c35b768689b41872378b388471cfc1d347cdcf229d7a661ef2fc2f2bc1f4c961b".to_string(),
                "ae4d19ebfb6a91c8e06b9e913db55fa54efb013adcb4e29407d5aafb03994a2071d67661020d4be7d6c1224b40ce3b4d".to_string(),
                "80d4b82c4586b2cb8c1f9db73f14868ab3e9e5b112d7e5c017a7f43c4b882a7c944c5b402a1de57f914cb549e874b59e".to_string(),
                "9930fcf39721c82e91ddc7fac50e28b445990faa3e2907111e107b74823e75b9bf886bde47174022444dc555dadc0a33".to_string(),
                "8d96c3fc4e590c208ea38c6da47a20c62e1f5f2530156e2045746d9eae13c637621feb5ba62cd8d9fedf10d80b5c44d2".to_string(),
                "a50aba1c945c8ea778549bdf90c1670555a82e01c67e6f182651d3848d17d1e52b1626497f0a5b325f6c793f4461e12d".to_string(),
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
