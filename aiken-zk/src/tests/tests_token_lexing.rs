use crate::compiler::lexer;
use crate::compiler::token_zk::TokenZK as Token;
use crate::tests::token_factory::{int_token, variable_token};
use crate::zk_examples::*;
// --------- Addition --------- //

#[test]
fn test_lexer_translates_public_addition_parameters_numeric() {
    let program = "offchain addition(pub 4, pub 5, pub 9)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Addition {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(4)
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(9)
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_private_addition_parameters_numeric() {
    let program = "offchain addition(priv, priv, priv)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Addition {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                res: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_addition_parameters_without_visibility_numeric() {
    let program = "offchain addition(   4  , 5   , 9  )  ";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Addition {
                lhs: InputZK {
                    visibility: None,
                    token: int_token(4)
                },
                rhs: InputZK {
                    visibility: None,
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: None,
                    token: int_token(9)
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_addition_parameters_with_mixed_visibility_and_input_types() {
    let program = "offchain addition(priv, pub 5, olga)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Addition {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: None,
                    token: variable_token("olga")
                },
            }
        },
        *offchain_token
    );
}

// --------- Subtraction --------- //

#[test]
fn test_lexer_translates_public_subtraction_parameters_numeric() {
    let program = "offchain subtraction(pub 4, pub 5, pub 9)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Subtraction {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(4)
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(9)
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_private_subtraction_parameters_numeric() {
    let program = "offchain subtraction(priv, priv, priv)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Subtraction {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                res: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_subtraction_parameters_without_visibility_numeric() {
    let program = "offchain subtraction(   4  , 5   , 9  )  ";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Subtraction {
                lhs: InputZK {
                    visibility: None,
                    token: int_token(4)
                },
                rhs: InputZK {
                    visibility: None,
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: None,
                    token: int_token(9)
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_subtraction_parameters_with_mixed_visibility_and_input_types() {
    let program = "offchain subtraction(priv, pub 5, olga)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Subtraction {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: None,
                    token: variable_token("olga")
                },
            }
        },
        *offchain_token
    );
}

// --------- Multiplication --------- //

#[test]
fn test_lexer_translates_multiplication_parameters_with_mixed_visibility_and_input_types() {
    let program = "offchain multiplication(priv, pub 5, olga)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Multiplication {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                res: InputZK {
                    visibility: None,
                    token: variable_token("olga")
                },
            }
        },
        *offchain_token
    );
}

// --------- Fibonacci --------- //

#[test]
fn test_lexer_translates_fibonacci_parameters_with_mixed_visibility_and_input_types() {
    let program = "offchain fibonacci(priv, pub 5, olga, pub 1)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::Fibonacci {
                fib_0: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                fib_1: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                n: InputZK {
                    visibility: None,
                    token: variable_token("olga")
                },
                res: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(1)
                },
            }
        },
        *offchain_token
    );
}

// --------- If --------- //

#[test]
fn test_lexer_translates_if_parameters_with_mixed_visibility_and_input_types() {
    let program = "offchain if(priv, pub 5, olga, pub 1)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::If {
                condition: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                assigned: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
                true_branch: InputZK {
                    visibility: None,
                    token: variable_token("olga")
                },
                false_branch: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(1)
                },
            }
        },
        *offchain_token
    );
}
// --------- AssertEq --------- //

#[test]
fn test_lexer_translates_assert_eq_parameters_with_mixed_visibility_and_input_types() {
    let program = "offchain assert_eq(priv, pub 5)";
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::AssertEq {
                lhs: InputZK {
                    visibility: Some(InputVisibility::Private),
                    token: None
                },
                rhs: InputZK {
                    visibility: Some(InputVisibility::Public),
                    token: int_token(5)
                },
            }
        },
        *offchain_token
    );
}

#[test]
fn test_lexer_translates_custom_circom() {
    let program = r#"offchain custom("path/to/circom/with/main.circom", [a, 5])"#;
    let lexer::LexInfo { tokens, .. } = lexer::Lexer::new().run(program).unwrap();
    let offchain_token = &tokens[0].0;
    assert_eq!(
        Token::Offchain {
            example: ZkExample::CustomCircom {
                path: String::from("path/to/circom/with/main.circom"),
                public_inputs: vec![variable_token("a").unwrap(), int_token(5).unwrap()]
            },
        },
        *offchain_token
    );
}

#[test]
#[should_panic(expected = "Private parameters cannot be followed by an identifier")]
fn test_lexer_does_not_allow_identifiers_for_private_parameters() {
    let program = "offchain assert_eq(priv a, pub 5)";
    let lexer::LexInfo { .. } = lexer::Lexer::new().run(program).unwrap();
}