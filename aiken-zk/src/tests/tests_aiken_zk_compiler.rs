use crate::aiken_zk_compiler::AikenZkCompiler;
use std::fs;

#[test]
fn test_compiler_can_save_source_code_into_file(){
    let source_code = source_code_addition();
    let source_code_path = manifest_path() + "/src/tests/artifacts/test.circom";

    let mut compiler = AikenZkCompiler::from(source_code.clone());
    let res = compiler.save_into_file(source_code_path.clone());

    assert!(res.is_ok());
    let stored_content = fs::read_to_string(&source_code_path).expect("No se pudo leer el archivo");
    assert_eq!(source_code, stored_content);
    assert_eq!(source_code_path, compiler.circom_source_code_path.unwrap());
}



fn manifest_path() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}

fn source_code_addition() -> String {
    r#"include "templates/addition.circom";
component main { public: [a,b,c] } = Addition();"#.to_string()
}