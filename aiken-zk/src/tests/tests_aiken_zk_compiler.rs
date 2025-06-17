use crate::aiken_zk_compiler::AikenZkCompiler;
use std::fs;
use std::env;

#[test]
fn test_compiler_can_save_source_code_into_file(){
    env::set_current_dir(&sandbox_path()).expect("Couldn't change directory");
    let source_code = source_code_addition();
    let source_code_path = "test.circom";

    let mut compiler = AikenZkCompiler::from(source_code.clone());
    let res = compiler.save_into_file(source_code_path.to_string());

    assert!(res.is_ok());
    let stored_content = fs::read_to_string(&source_code_path).expect("No se pudo leer el archivo");
    assert_eq!(source_code, stored_content);
    assert_eq!(source_code_path, compiler.circom_source_code_path.unwrap());

    clear_sandbox()
}

#[test]
fn test_compiler_can_compile_the_generated_circom_component(){
    env::set_current_dir(&sandbox_path()).expect("Couldn't change directory");
    let source_code = source_code_addition();
    let circom_program_filename = "test.circom".to_string();
    let mut compiler = AikenZkCompiler::from(source_code.clone());
    compiler.save_into_file(circom_program_filename.clone()).unwrap();

    let res = compiler.create_verification_key(circom_program_filename);

    assert!(res.is_ok());
    let stored_vk = fs::read_to_string("build/verification_key.json").expect("No se pudo leer el archivo");
    assert!(stored_vk.contains("protocol"));

    clear_sandbox();
}

fn sandbox_path() -> String {
    manifest_path() + "/src/tests/sandbox/"
}

fn manifest_path() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}

fn clear_sandbox() {
    let _ = fs::remove_file("test.circom");
    let _ = fs::remove_dir_all("build");
}

fn source_code_addition() -> String {
    r#"include "templates/addition.circom";
component main { public [a,b,c] } = Addition();"#.to_string()
}