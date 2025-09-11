use crate::compiler::token_zk::TokenZK as Token;
use aiken_lang::parser::error::ParseError;
use aiken_lang::parser::token::Base;
use chumsky::primitive::{choice, just, one_of};
use chumsky::{Parser, text};
use num_bigint::BigInt;

pub fn base_10_parser() -> impl Parser<char, Token, Error = ParseError> {
    text::int(10).map(|value| Token::Int {
        value,
        base: Base::Decimal {
            numeric_underscore: false,
        },
    })
}

pub fn base_10_underscore_parser() -> impl Parser<char, Token, Error = ParseError> {
    one_of("0123456789")
        .repeated()
        .at_least(1)
        .at_most(3)
        .separated_by(just("_"))
        .at_least(2)
        .flatten()
        .collect::<String>()
        .map(|value| Token::Int {
            value,
            base: Base::Decimal {
                numeric_underscore: true,
            },
        })
}

pub fn base_16_parser() -> impl Parser<char, Token, Error = ParseError> {
    just("0x")
        .ignore_then(
            one_of("0123456789abcdefABCDEF")
                .repeated()
                .at_least(1)
                .collect::<String>(),
        )
        .validate(|value: String, span, emit| {
            let value = match BigInt::parse_bytes(value.as_bytes(), 16) {
                None => {
                    emit(ParseError::malformed_base16_digits(span));
                    String::new()
                }
                Some(n) => n.to_str_radix(10),
            };

            Token::Int {
                value,
                base: Base::Hexadecimal,
            }
        })
}

pub fn int_parser() -> impl Parser<char, Token, Error = ParseError> {
    choice((
        base_16_parser(),
        base_10_underscore_parser(),
        base_10_parser(),
    ))
}
