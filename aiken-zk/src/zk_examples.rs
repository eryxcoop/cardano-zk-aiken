use chumsky::prelude::*;
use std::fmt;
use std::fmt::Formatter;

use crate::compiler::parsers::int_parser;
use crate::compiler::token_zk::{TokenZK as Token, TokenZK};
use crate::zk_examples::TokenWithCardinality::{Multiple, Single};
use aiken_lang::parser::error::ParseError;
use chumsky::{
    prelude::{choice, just},
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputVisibility {
    Public,
    Private,
}

impl InputVisibility {
    pub fn from(keyword: Option<&str>) -> Option<Self> {
        match keyword {
            Some("pub") => Some(Self::Public),
            Some("priv") => Some(Self::Private),
            None => None,
            _ => panic!("Visibility not recognized"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenWithCardinality {
    Single(Token),
    Multiple(Token),
}

impl fmt::Display for TokenWithCardinality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Single(token) => write!(f, "{}", token),
            Multiple(token) => write!(f, "{}", token),
        }
    }
}

impl TokenWithCardinality {
    pub fn new_single(token: Token) -> Self {
        Single(token)
    }

    pub fn new_multiple(token: Token) -> Self {
        Multiple(token)
    }

    pub fn extract_single(&self) -> Option<TokenZK> {
        match self {
            Single(token) => Some(token.clone()),
            Multiple(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CircuitTemplateParameter {
    pub token: Box<Token>,
}

impl CircuitTemplateParameter {
    pub fn from(visibility_token: (Option<InputVisibility>, Option<Token>)) -> Self {
        let (maybe_input_visibility, maybe_token) = visibility_token;

        if maybe_input_visibility.is_some() {
            panic!("A circuit template parameter cannot have visibility");
        }

        let identifier_token = if let Some(Token::Int { value, base }) = maybe_token {
            Box::new(TokenZK::Int { value, base })
        } else {
            panic!("A circuit template parameter must be constant");
        };

        Self {
            token: identifier_token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputZK {
    pub visibility: InputVisibility,
    pub token: Option<Box<TokenWithCardinality>>,
}

impl InputZK {
    pub fn from(visibility_token: (Option<InputVisibility>, Option<TokenZK>)) -> Self {
        let (maybe_input_visibility, maybe_cardinality_token) = visibility_token;

        match maybe_input_visibility {
            None | Some(InputVisibility::Public) => {
                if maybe_cardinality_token.is_none() {
                    panic!("Public parameters must be followed by an identifier");
                }

                Self {
                    visibility: InputVisibility::Public,
                    token: Some(Box::new(TokenWithCardinality::new_single(
                        maybe_cardinality_token.unwrap(),
                    ))),
                }
            }
            Some(InputVisibility::Private) => {
                if maybe_cardinality_token.is_some() {
                    panic!("Private parameters cannot be followed by an identifier");
                }

                Self {
                    visibility: InputVisibility::Private,
                    token: None,
                }
            }
        }
    }
}

impl fmt::Display for InputZK {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.clone().unwrap())
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
        n: CircuitTemplateParameter,
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
    Sha256 {
        n_bits: CircuitTemplateParameter,
        r#in: InputZK,
        out: InputZK,
    },
    CustomCircom {
        path: String,
        public_inputs: Vec<Box<TokenWithCardinality>>,
    },
    Poseidon {
        n_inputs: CircuitTemplateParameter,
        r#in: InputZK,
        out: InputZK,
    },
    MerkleTreeChecker {
        levels: CircuitTemplateParameter,
        leaf: InputZK,
        root: InputZK,
        path_elements: InputZK,
        path_indices: InputZK,
    },
    PolynomialEvaluations {
        grade: CircuitTemplateParameter,
        coefficients: InputZK,
        amount_of_evaluations: CircuitTemplateParameter,
        domain: InputZK,
        evaluations: InputZK,
    },
}

impl ZkExample {
    fn name_parser() -> impl Parser<char, Token, Error = ParseError> {
        text::ident().map(|name| Token::Name { name }).padded()
    }

    fn name_parser_cardinality() -> impl Parser<char, TokenWithCardinality, Error = ParseError> {
        choice((
            Self::name_parser().map(|id| { TokenWithCardinality::new_single(id)}),
            just("@").then(Self::name_parser()).map(|(_, id)|{TokenWithCardinality::new_multiple(id)})
        ))
    }

    fn int_or_var()
    -> impl Parser<char, (Option<InputVisibility>, Option<Token>), Error = ParseError> {
        choice((just("priv"), just("pub")))
            .padded()
            .or_not()
            .then(choice((int_parser(), Self::name_parser())).or_not())
            .map(|(maybe_visibility, maybe_token)| {
                (InputVisibility::from(maybe_visibility), maybe_token)
            })
    }

    fn parameters(
        ammount_of_parameters: usize,
    ) -> impl Parser<char, Vec<(Option<InputVisibility>, Option<Token>)>, Error = ParseError> {
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
                    n: CircuitTemplateParameter::from(args[2].clone()),
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

    fn sha256_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("sha256")
            .padded()
            .ignore_then(Self::parameters(3))
            .map(|args| Token::Offchain {
                example: ZkExample::Sha256 {
                    n_bits: CircuitTemplateParameter::from(args[0].clone()),
                    r#in: InputZK::from(args[1].clone()),
                    out: InputZK::from(args[2].clone()),
                },
            })
    }

    fn poseidon_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("poseidon")
            .padded()
            .ignore_then(Self::parameters(3))
            .map(|args| Token::Offchain {
                example: ZkExample::Poseidon {
                    n_inputs: CircuitTemplateParameter::from(args[0].clone()),
                    r#in: InputZK::from(args[1].clone()),
                    out: InputZK::from(args[2].clone()),
                },
            })
    }

    fn merkle_tree_checker_parser() -> impl Parser<char, Token, Error = ParseError> {
        just("merkle_tree_checker")
            .padded()
            .ignore_then(Self::parameters(5))
            .map(|args| Token::Offchain {
                example: ZkExample::MerkleTreeChecker {
                    levels: CircuitTemplateParameter::from(args[0].clone()),
                    leaf: InputZK::from(args[1].clone()),
                    root: InputZK::from(args[2].clone()),
                    path_elements: InputZK::from(args[3].clone()),
                    path_indices: InputZK::from(args[4].clone()),
                },
            })
    }

    fn custom_circom_parser() -> impl Parser<char, Token, Error = ParseError> {
        let string_literal_parser = just('"')
            .ignore_then(filter(|c| *c != '"').repeated().collect::<String>())
            .then_ignore(just('"'));
        let identifiers_parser = choice((
            int_parser().map(|literal_token| TokenWithCardinality::new_single(literal_token)),
            Self::name_parser_cardinality(),
        ));

        let public_input_identifiers_list_parser = identifiers_parser
            .separated_by(just(",").padded())
            .delimited_by(just("[").padded(), just("]").padded());
        let custom_circom_argument_parser = string_literal_parser
            .padded()
            .then_ignore(just(",").padded())
            .then(public_input_identifiers_list_parser)
            .delimited_by(
                just("(").padded(),
                just(")").padded_by(just(" ").repeated()),
            );

        just("custom")
            .padded()
            .ignore_then(custom_circom_argument_parser)
            .map(|(path, public_input_identifiers)| Token::Offchain {
                example: ZkExample::CustomCircom {
                    path,
                    public_inputs: public_input_identifiers
                        .iter()
                        .map(|token| Box::new(token.clone()))
                        .collect(),
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
            Self::sha256_parser(),
            Self::poseidon_parser(),
            Self::merkle_tree_checker_parser(),
            Self::polynomial_evaluations(),
            Self::custom_circom_parser(),
        ))
    }

    fn polynomial_evaluations() -> impl Parser<char, Token, Error = ParseError> {
        just("polynomial_evaluations")
            .padded()
            .ignore_then(Self::parameters(5))
            .map(|args| Token::Offchain {
                example: ZkExample::PolynomialEvaluations {
                    grade: CircuitTemplateParameter::from(args[0].clone()),
                    coefficients: InputZK::from(args[1].clone()),
                    amount_of_evaluations: CircuitTemplateParameter::from(args[2].clone()),
                    domain: InputZK::from(args[3].clone()),
                    evaluations: InputZK::from(args[4].clone()),
                },
            })
    }
}
