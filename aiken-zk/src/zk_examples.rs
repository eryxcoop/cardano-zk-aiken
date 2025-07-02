use chumsky::prelude::*;
use std::fmt;
use std::fmt::Formatter;

use crate::parsers::int_parser;
use crate::token_zk::TokenZK as Token;
use aiken_lang::parser::error::ParseError;
use chumsky::{
    Parser,
    prelude::{choice, just},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputVisibility {
    Public,
    Private,
}

impl InputVisibility {
    pub fn from(keyword: &str) -> Self {
        match keyword {
            "pub" => Self::Public,
            "priv" => Self::Private,
            _ => panic!("Visibility not recognized"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputZK {
    pub token: Box<Token>,
    pub visibility: Option<InputVisibility>,
}

impl InputZK {
    pub fn from(visibility_token: (Option<InputVisibility>, Token)) -> Self {
        Self {
            token: Box::new(visibility_token.1),
            visibility: visibility_token.0,
        }
    }
}

impl fmt::Display for InputZK {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZkExample {
    Addition {
        lhs: InputZK,
        rhs: InputZK,
        res: InputZK,
    },
    Subtraction {
        lhs: InputZK,
        rhs: InputZK,
        res: InputZK,
    },
    Multiplication {
        lhs: InputZK,
        rhs: InputZK,
        res: InputZK,
    },
    Fibonacci {
        fib_0: InputZK,
        fib_1: InputZK,
        n: InputZK,
        res: InputZK,
    },
    If {
        condition: InputZK,
        assigned: InputZK,
        true_branch: InputZK,
        false_branch: InputZK,
    },
    AssertEq {
        lhs: InputZK,
        rhs: InputZK,
    },
}

impl ZkExample {
    fn name_parser() -> impl Parser<char, Token, Error = ParseError> {
        text::ident().map(|name| Token::Name { name }).padded()
    }

    fn int_or_var() -> impl Parser<char, (Option<InputVisibility>, Token), Error = ParseError> {
        Self::visibility_parser()
            .or_not()
            .then(choice((int_parser(), Self::name_parser())))
    }

    pub fn visibility_parser() -> impl Parser<char, InputVisibility, Error = ParseError> {
        choice((just("pub").padded(), just("priv").padded()))
            .map(|visibility_keyword| InputVisibility::from(visibility_keyword))
    }

    fn parameters(
        ammount_of_parameters: usize,
    ) -> impl Parser<char, Vec<(Option<InputVisibility>, Token)>, Error = ParseError> {
        Self::int_or_var()
            .separated_by(just(',').padded())
            .exactly(ammount_of_parameters)
            .delimited_by(
                just('(').padded(),
                just(' ').repeated().ignored().then(just(')')),
            )
    }

    fn addition_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("addition")
            .padded()
            .ignore_then(Self::parameters(3))
            .map(|args| Token::Offchain {
                example: ZkExample::Addition {
                    lhs: InputZK::from(args[0].clone()),
                    rhs: InputZK::from(args[1].clone()),
                    res: InputZK::from(args[2].clone()),
                },
            })
    }

    fn subtraction_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("subtraction")
            .padded()
            .ignore_then(Self::parameters(3))
            .map(|args| Token::Offchain {
                example: ZkExample::Subtraction {
                    lhs: InputZK::from(args[0].clone()),
                    rhs: InputZK::from(args[1].clone()),
                    res: InputZK::from(args[2].clone()),
                },
            })
    }

    fn multiplication_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("multiplication")
            .padded()
            .ignore_then(Self::parameters(3))
            .map(|args| Token::Offchain {
                example: ZkExample::Multiplication {
                    lhs: InputZK::from(args[0].clone()),
                    rhs: InputZK::from(args[1].clone()),
                    res: InputZK::from(args[2].clone()),
                },
            })
    }

    fn fibonacci_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("fibonacci")
            .padded()
            .ignore_then(Self::parameters(4))
            .map(|args| Token::Offchain {
                example: ZkExample::Fibonacci {
                    fib_0: InputZK::from(args[0].clone()),
                    fib_1: InputZK::from(args[1].clone()),
                    n: InputZK::from(args[2].clone()),
                    res: InputZK::from(args[3].clone()),
                },
            })
    }

    fn if_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("if")
            .padded()
            .ignore_then(Self::parameters(4))
            .map(|args| Token::Offchain {
                example: ZkExample::If {
                    condition: InputZK::from(args[0].clone()),
                    assigned: InputZK::from(args[1].clone()),
                    true_branch: InputZK::from(args[2].clone()),
                    false_branch: InputZK::from(args[3].clone()),
                },
            })
    }

    fn assert_eq_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("assert_eq")
            .padded()
            .ignore_then(Self::parameters(2))
            .map(|args| Token::Offchain {
                example: ZkExample::AssertEq {
                    lhs: InputZK::from(args[0].clone()),
                    rhs: InputZK::from(args[1].clone()),
                },
            })
    }

    pub fn parser() -> impl Parser<char, Token, Error = ParseError> {
        choice((
            Self::addition_parser(),
            Self::subtraction_parser(),
            Self::multiplication_parser(),
            Self::fibonacci_parser(),
            Self::if_parser(),
            Self::assert_eq_parser(),
        ))
    }
}
