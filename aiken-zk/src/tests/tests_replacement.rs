use crate::create_zk_src;

#[test]
fn test_empty_replacement() {
    let program_without_offchain = String::from("int a = 4");
    assert_eq!(
        program_without_offchain,
        create_zk_src(&program_without_offchain)
    );
}

#[test]
fn test_single_replacement_for_addition_with_constant_params() {
    let program_only_offchain = "offchain addition(4, 5, 9)";
    assert_eq!("zk[4+5=9]", create_zk_src(&program_only_offchain));
}

#[test]
fn test_single_replacement_for_addition_with_constant_params_in_context() {
    let program_only_offchain = "let a = 4\noffchain addition(4, 5, 9)\nlet a = 4";
    assert_eq!(
        "let a = 4\nzk[4+5=9]\nlet a = 4",
        create_zk_src(&program_only_offchain)
    );
}

#[test]
fn test_single_replacement_for_subtraction_with_constant_params() {
    let program_only_offchain = "offchain subtraction(10, 5, 5)";
    assert_eq!("zk[10-5=5]", create_zk_src(&program_only_offchain));
}

#[test]
fn test_single_replacement_for_subtraction_with_constant_params_in_context() {
    let program_only_offchain = "let a = 4\noffchain subtraction(10, 5, 5)\nlet a = 4";
    assert_eq!(
        "let a = 4\nzk[10-5=5]\nlet a = 4",
        create_zk_src(&program_only_offchain)
    );
}

#[test]
fn test_single_replacement_for_multiplication_with_constant_params() {
    let program_only_offchain = "offchain multiplication(4, 5, 20)";
    assert_eq!("zk[4*5=20]", create_zk_src(&program_only_offchain));
}

#[test]
fn test_single_replacement_for_multiplication_with_constant_params_in_context() {
    let program_only_offchain = "let a = 4\noffchain multiplication(4, 5, 20)\nlet a = 4";
    assert_eq!(
        "let a = 4\nzk[4*5=20]\nlet a = 4",
        create_zk_src(&program_only_offchain)
    );
}

#[test]
fn test_single_replacement_for_fibonacci_with_constant_params() {
    let program_only_offchain = "offchain fibonacci(4, 5, 3, 9)";
    assert_eq!(
        "zk[fib-4-5...3 times =9]",
        create_zk_src(&program_only_offchain)
    );
}

#[test]
fn test_single_replacement_for_fibonacci_with_constant_params_in_context() {
    let program_only_offchain = "let a = 4\noffchain fibonacci(4, 5, 3, 9)\nlet a = 4";
    assert_eq!(
        "let a = 4\nzk[fib-4-5...3 times =9]\nlet a = 4",
        create_zk_src(&program_only_offchain)
    );
}

#[test]
fn test_single_replacement_for_if_with_constant_params() {
    let program_only_offchain = "offchain if(true, a, a, 9)";
    assert_eq!(
        "zk[a = if true then a else 9]",
        create_zk_src(&program_only_offchain)
    );
}

#[test]
fn test_single_replacement_for_if_with_constant_params_in_context() {
    let program_only_offchain = "let a = 4\noffchain if(false, 4, 3, 4)\nlet a = 4";
    assert_eq!(
        "let a = 4\nzk[4 = if false then 3 else 4]\nlet a = 4",
        create_zk_src(&program_only_offchain)
    );
}
