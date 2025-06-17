use crate::aiken_zk_compiler::AikenZkCompiler;
use std::fs;
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tempfile::{tempdir, TempDir};
use fs_extra::dir::{copy, CopyOptions};


#[test]
fn test_compiler_can_save_source_code_into_file(){
    let _temp_dir = create_sandbox_and_set_as_current_directory();
    let source_code = source_code_addition();
    let source_code_path = "test.circom";

    let mut compiler = AikenZkCompiler::from(source_code.clone());
    compiler.save_into_file(source_code_path.to_string()).unwrap();

    let stored_content = fs::read_to_string(&source_code_path).expect("No se pudo leer el archivo");
    assert_eq!(source_code, stored_content);
    assert_eq!(source_code_path, compiler.circom_source_code_path.unwrap());
}

#[test]
fn test_compiler_can_compile_the_generated_circom_component(){
    let _ = create_sandbox_and_set_as_current_directory();
    let circom_program_filename = "test.circom".to_string();
    let mut compiler = AikenZkCompiler::from(source_code_addition());
    compiler.save_into_file(circom_program_filename.clone()).unwrap();

    let res = compiler.create_verification_key(circom_program_filename);

    assert!(res.is_ok());
    let stored_vk = fs::read_to_string("build/verification_key.json").expect("No se pudo leer el archivo");
    assert!(stored_vk.contains("protocol"));
}

fn create_sandbox_and_set_as_current_directory() -> TempDir {
    let sandbox_path = &sandbox_path();
    let source = Path::new(sandbox_path);
    let temp_dir = tempdir().expect("Could not create temp dir");
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    copy(source, temp_dir.path(), &options).expect("Could not copy template into directory");
    env::set_current_dir(temp_dir.path()).expect("Couldn't move to temp directory");

    let perms = fs::Permissions::from_mode(0o777);
    fs::set_permissions(&temp_dir.path(), perms).expect("Failed to set permissions");

    temp_dir
}

fn sandbox_path() -> String {
    manifest_path() + "/src/tests/sandbox/"
}

fn manifest_path() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}

fn source_code_addition() -> String {
    r#"include "templates/addition.circom";
component main { public [a,b,c] } = Addition();"#.to_string()
}