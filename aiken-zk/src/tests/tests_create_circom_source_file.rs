use crate::component_creator::ComponentCreator;
use crate::tests::token_factory::{addition_token_with_all_private_inputs, addition_token_with_mixed_visibility, addition_token_with_public_inputs, assert_eq_token_with_mixed_visibility, fibonacci_token_with_mixed_visibility, if_token_with_mixed_visibility, merkle_tree_checker_token_with_mixed_visibility, multiplication_token_with_mixed_visibility, polynomial_evaluations_token, poseidon_token_with_mixed_visibility, sha256_token_with_mixed_visibility, subtraction_token_with_mixed_visibility};

#[test]
fn test_circom_version_should_be_2_1_9() {
    let token = addition_token_with_public_inputs();
    let component_creator = ComponentCreator::from_token(token);
    let output = component_creator.create();
    let lines = output.lines().collect::<Vec<_>>();
    assert_eq!("pragma circom 2.1.9;", lines[0])
}

// ----------- ADDITION ----------- //

#[test]
fn test_component_creator_can_create_addition_component_with_all_public_inputs() {
    let token = addition_token_with_public_inputs();
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main { public [a,b,c] } = Addition();"#;
    assert_eq!(expected_program, component_creator.create())
}

#[test]
fn test_component_creator_can_create_addition_component_with_all_private_inputs() {
    let token = addition_token_with_all_private_inputs();
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main = Addition();"#;
    assert_eq!(expected_program, component_creator.create())
}

#[test]
fn test_component_creator_can_create_addition_component_with_mixed_visibilities() {
    let token = addition_token_with_mixed_visibility();
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/addition.circom";
component main { public [b,c] } = Addition();"#;
    assert_eq!(expected_program, component_creator.create())
}

// ----------- SUBTRACTION ----------- //

#[test]
fn test_component_creator_can_create_subtraction_component_with_mixed_visibilities() {
    let token_public_addition = subtraction_token_with_mixed_visibility();
    let component_creator = ComponentCreator::from_token(token_public_addition);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/subtraction.circom";
component main { public [a,c] } = Subtraction();"#;
    assert_eq!(expected_program, component_creator.create())
}

// ----------- MULTIPLICATION ----------- //

#[test]
fn test_component_creator_can_create_multiplication_component_with_mixed_visibilities() {
    let token = multiplication_token_with_mixed_visibility();
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/multiplication.circom";
component main { public [a,c] } = Multiplication();"#;
    assert_eq!(expected_program, component_creator.create())
}

// ----------- FIBONACCI ----------- //

#[test]
fn test_component_creator_can_create_fibonacci_component_with_mixed_visibilities() {
    let length_of_fibonacci_series = 5;
    let token = fibonacci_token_with_mixed_visibility(length_of_fibonacci_series);
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = format!(r#"pragma circom 2.1.9;
include "templates/fibonacci.circom";
component main {{ public [a,c] }} = Fibonacci({length_of_fibonacci_series});"#);
    assert_eq!(expected_program, component_creator.create())
}

// ----------- IF ----------- //

#[test]
fn test_component_creator_can_create_if_component_with_mixed_visibilities() {
    let token = if_token_with_mixed_visibility();
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/if.circom";
component main { public [condition,true_branch,false_branch] } = If();"#;
    assert_eq!(expected_program, component_creator.create())
}

// ----------- ASSERT_EQ ----------- //

#[test]
fn test_component_creator_can_create_assert_eq_component_with_mixed_visibilities() {
    let token = assert_eq_token_with_mixed_visibility();
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = r#"pragma circom 2.1.9;
include "templates/assert_eq.circom";
component main { public [b] } = AssertEq();"#;
    assert_eq!(expected_program, component_creator.create())
}

// ----------- SHA256 ----------- //

#[test]
fn test_component_creator_can_create_sha256_component_with_mixed_visibilities() {
    let length_of_input = 5;
    let token = sha256_token_with_mixed_visibility(length_of_input);
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = format!(r#"pragma circom 2.1.9;
include "templates/hash.circom";
component main {{ public [out] }} = Sha256({length_of_input});"#);
    assert_eq!(expected_program, component_creator.create())
}

// ----------- POSEIDON ----------- //

#[test]
fn test_component_creator_can_create_poseidon_component() {
    let length_of_input = 5;
    let token = poseidon_token_with_mixed_visibility(length_of_input);
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = format!(r#"pragma circom 2.1.9;
include "templates/hash.circom";
component main {{ public [out] }} = Poseidon({length_of_input});"#);
    assert_eq!(expected_program, component_creator.create())
}

// ----------- MERKLE_TREE_CHECKER ----------- //

#[test]
fn test_component_creator_can_create_merkle_tree_checker_component() {
    let tree_height = 3;
    let token = merkle_tree_checker_token_with_mixed_visibility(tree_height);
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = format!(r#"pragma circom 2.1.9;
include "templates/merkle_tree_checker.circom";
component main {{ public [leaf,root,pathElements,pathIndices] }} = MerkleTreeChecker({tree_height});"#);
    assert_eq!(expected_program, component_creator.create())
}

// ----------- POLYNOMIAL_EVALUATIONS ----------- //

#[test]
fn test_component_creator_can_create_polynomial_evaluations_component() {
    let grade = 2;
    let amount_of_evaluations = 2;
    let token = polynomial_evaluations_token(grade, amount_of_evaluations);
    let component_creator = ComponentCreator::from_token(token);
    let expected_program = format!(r#"pragma circom 2.1.9;
include "templates/polynomials.circom";
component main {{ public [coefficients,domain,evaluations] }} = PolynomialEvaluations({grade}, {amount_of_evaluations});"#);
    assert_eq!(expected_program, component_creator.create())
}
