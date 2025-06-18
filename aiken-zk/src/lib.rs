pub mod lexer;
pub mod parsers;
pub mod token_zk;
pub mod zk_examples;

#[cfg(test)]
mod tests;

pub mod component_creator;
pub mod circom_compiler;
pub mod aiken_zk_compiler;

use crate::token_zk::TokenZK;
use crate::zk_examples::ZkExample;
use lexer::Lexer;

pub fn deprecated_replace_offchain_by_zk_in_src(src: &str) -> String {
    let mut src_zk = String::from(src);

    let lexer::LexInfo { tokens, .. } = Lexer::new().run(src).unwrap();
    println!("Tokens: {:?}", tokens);

    for (token, span) in tokens {
        match token {
            TokenZK::Offchain { example } => {
                let replacement = ZkExample::keyword_to_replacement(example);
                src_zk.replace_range(span.start..span.end, &replacement);
            }
            _ => {}
        }
    }

    src_zk
}
