use crate::compiler::token_zk;
use crate::zk_examples::{InputVisibility, InputZK, ZkExample};
use aiken_lang::parser::token::Base;

pub fn int_token(n: u32) -> Box<token_zk::TokenZK> {
    Box::new(token_zk::TokenZK::Int {
        value: n.to_string(),
        base: Base::Decimal {
            numeric_underscore: false,
        },
    })
}

pub fn variable_token(s: &str) -> Box<token_zk::TokenZK> {
    Box::new(token_zk::TokenZK::Name {
        name: s.to_string(),
    })
}

pub fn addition_token_with_public_inputs() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Addition {
            lhs: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("c"),
            },
            rhs: InputZK {
                visibility: Some(InputVisibility::Public),
                token: int_token(5),
            },
            res: InputZK {
                visibility: None,
                token: int_token(9),
            },
        },
    }
}

pub fn addition_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Addition {
            lhs: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("c"),
            },
            rhs: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("b"),
            },
            res: InputZK {
                visibility: None,
                token: variable_token("n"),
            },
        },
    }
}

pub fn addition_token_with_all_private_inputs() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Addition {
            lhs: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("c"),
            },
            rhs: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("b"),
            },
            res: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("n"),
            },
        },
    }
}

// ----- SUBTRACTION ----- //

pub fn subtraction_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Subtraction {
            lhs: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("c"),
            },
            rhs: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("b"),
            },
            res: InputZK {
                visibility: None,
                token: variable_token("n"),
            },
        },
    }
}

// ----- MULTIPLICATION ----- //

pub fn multiplication_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Multiplication {
            lhs: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("c"),
            },
            rhs: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("b"),
            },
            res: InputZK {
                visibility: None,
                token: variable_token("n"),
            },
        },
    }
}

// ----- FIBONACCI ----- //

pub fn fibonacci_token_with_mixed_visibility(n: usize) -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::Fibonacci {
            fib_0: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("c"),
            },
            fib_1: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("c"),
            },
            n: InputZK {
                visibility: Some(InputVisibility::Public),
                token: int_token(n as u32),
            },
            res: InputZK {
                visibility: None,
                token: variable_token("n"),
            },
        },
    }
}

// ----- IF ----- //

pub fn if_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::If {
            condition: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("a"),
            },
            assigned: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("b"),
            },
            true_branch: InputZK {
                visibility: Some(InputVisibility::Public),
                token: variable_token("c"),
            },
            false_branch: InputZK {
                visibility: None,
                token: variable_token("d"),
            },
        },
    }
}

// ----- ASSERT_EQ ----- //

pub fn assert_eq_token_with_mixed_visibility() -> token_zk::TokenZK {
    token_zk::TokenZK::Offchain {
        example: ZkExample::AssertEq {
            lhs: InputZK {
                visibility: Some(InputVisibility::Private),
                token: variable_token("a"),
            },
            rhs: InputZK {
                visibility: Some(InputVisibility::Public),
                token: int_token(5),
            },
        },
    }
}
