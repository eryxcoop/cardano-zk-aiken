use serial_test::serial;
use crate::aiken_zk_compiler::AikenZkCompiler;
use crate::tests::aiken_program_factory::aiken_template_with_keyword;
use crate::tests::utils::create_sandbox_and_set_as_current_directory;

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_public_variables_by_the_corresponding_funcion_and_call(){
    test_compiler_can_replace_addition_by_the_corresponding_funcion_and_call(
        "offchain addition(pub a, pub b, pub c)",
        "zk_verify_or_fail(redeemer, [a, b, c])"
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_private_variables_by_the_corresponding_funcion_and_call(){
    test_compiler_can_replace_addition_by_the_corresponding_funcion_and_call(
        "offchain addition(priv a, priv b, priv c)",
        "zk_verify_or_fail(redeemer, [])"
    );
}

#[test]
#[serial]
fn test_compiler_can_replace_addition_of_mixed_variables_by_the_corresponding_funcion_and_call(){
    test_compiler_can_replace_addition_by_the_corresponding_funcion_and_call(
        "offchain addition(priv a, b, pub c)",
        "zk_verify_or_fail(redeemer, [b, c])"
    );
}

fn test_compiler_can_replace_addition_by_the_corresponding_funcion_and_call(original: &str, replacement: &str){
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let aiken_src = aiken_template_with_keyword(original);
    let aiken_src_filename = "my_program".to_string();

    let aiken_zk_src = AikenZkCompiler::apply_modifications_to_src_for_token(aiken_src, aiken_src_filename);

    let expected_aiken_src = aiken_template_with_keyword(replacement);
    assert_eq!(expected_aiken_src, aiken_zk_src);
}
