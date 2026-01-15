use crate::compiler::token_zk;
use crate::zk_examples::{
    CircuitTemplateParameter, InputVisibility, InputZK, TokenWithCardinality, ZkExample,
};
use aiken_lang::parser::token::Base;

pub fn int_token(n: u32) -> Option<Box<TokenWithCardinality>> {
    Some(Box::new(TokenWithCardinality::Single(
        token_zk::TokenZK::Int {
            value: n.to_string(),
            base: Base::Decimal {
                numeric_underscore: false,
            },
        },
    )))
}

pub fn single_variable_token(s: &str) -> Option<Box<TokenWithCardinality>> {
    Some(Box::new(TokenWithCardinality::new_single(
        token_zk::TokenZK::Name {
            name: s.to_string(),
        },
    )))
}

pub fn multiple_variable_token(s: &str) -> Option<Box<TokenWithCardinality>> {
    Some(Box::new(TokenWithCardinality::new_multiple(
        token_zk::TokenZK::Name {
            name: s.to_string(),
        },
    )))
}

pub fn addition_token_with_public_inputs() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Addition {
            lhs: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("c"),
            },
            rhs: InputZK {
                visibility: InputVisibility::Public,
                token: int_token(5),
            },
            res: InputZK {
                visibility: InputVisibility::Public,
                token: int_token(9),
            },
        },
    }
}

pub fn addition_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Addition {
            lhs: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            rhs: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("b"),
            },
            res: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("n"),
            },
        },
    }
}

pub fn addition_token_with_all_private_inputs() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Addition {
            lhs: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            rhs: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            res: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
        },
    }
}

// ----- SUBTRACTION ----- //

pub fn subtraction_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Subtraction {
            lhs: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("c"),
            },
            rhs: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            res: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("n"),
            },
        },
    }
}

// ----- MULTIPLICATION ----- //

pub fn multiplication_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Multiplication {
            lhs: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("c"),
            },
            rhs: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            res: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("n"),
            },
        },
    }
}

// ----- FIBONACCI ----- //

pub fn fibonacci_token_with_mixed_visibility(n: usize) -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Fibonacci {
            fib_0: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("c"),
            },
            fib_1: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            n: CircuitTemplateParameter {
                token: Box::new(int_token(n as u32).unwrap().extract_single().unwrap()),
            },
            res: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("n"),
            },
        },
    }
}

// ----- IF ----- //

pub fn if_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::If {
            condition: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("a"),
            },
            assigned: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            true_branch: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("c"),
            },
            false_branch: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("d"),
            },
        },
    }
}

// ----- ASSERT_EQ ----- //

pub fn assert_eq_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::AssertEq {
            lhs: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            rhs: InputZK {
                visibility: InputVisibility::Public,
                token: int_token(5),
            },
        },
    }
}

// ----- SHA256 ----- //

pub fn sha256_token_with_mixed_visibility(n: u32) -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Sha256 {
            n_bits: CircuitTemplateParameter {
                token: Box::new(int_token(n).unwrap().extract_single().unwrap()),
            },
            r#in: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            out: InputZK {
                visibility: InputVisibility::Public,
                token: int_token(n as u32),
            },
        },
    }
}

// ----- POSEIDON ----- //

pub fn poseidon_token_with_mixed_visibility(n: u32) -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Poseidon {
            n_inputs: CircuitTemplateParameter {
                token: Box::new(int_token(n).unwrap().extract_single().unwrap()),
            },
            r#in: InputZK {
                visibility: InputVisibility::Private,
                token: None,
            },
            out: InputZK {
                visibility: InputVisibility::Public,
                token: int_token(n as u32),
            },
        },
    }
}

// ----- MERKLE_TREE_CHECKER ----- //

pub fn merkle_tree_checker_token_with_mixed_visibility(n: u32) -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::MerkleTreeChecker {
            levels: CircuitTemplateParameter {
                token: Box::new(int_token(n).unwrap().extract_single().unwrap()),
            },
            leaf: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("leaf"),
            },
            root: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("root"),
            },
            path_elements: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("pathElements"),
            },
            path_indices: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("pathIndices"),
            },
        },
    }
}

// ----- POLYNOMIAL_EVALUATIONS ----- //

pub fn polynomial_evaluations_token(grade: u32, amount_of_evaluations: u32) -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::PolynomialEvaluations {
            grade: CircuitTemplateParameter {
                token: Box::new(int_token(grade).unwrap().extract_single().unwrap()),
            },
            coefficients: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("coefficients"),
            },
            amount_of_evaluations: CircuitTemplateParameter {
                token: Box::new(
                    int_token(amount_of_evaluations)
                        .unwrap()
                        .extract_single()
                        .unwrap(),
                ),
            },
            domain: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("domain"),
            },
            evaluations: InputZK {
                visibility: InputVisibility::Public,
                token: single_variable_token("evaluations"),
            },
        },
    }
}
