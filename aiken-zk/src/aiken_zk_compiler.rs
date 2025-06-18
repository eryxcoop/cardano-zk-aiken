use aiken_lang::ast::Span;
use crate::token_zk::TokenZK as Token;
use crate::zk_examples::ZkExample;

struct AikenZkCompiler;

impl AikenZkCompiler {
    pub fn find_offchain_token(tokens: Vec<(Token, Span)>) -> (Token, Span) {
        tokens.iter().find(|(token, span)| matches!(token, Token::Offchain {example})).unwrap().clone()
    }
}