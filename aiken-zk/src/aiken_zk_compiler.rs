use crate::circom_compiler::CircomCompiler;
use crate::component_creator::ComponentCreator;
use crate::lexer::{LexInfo, Lexer};
use crate::token_zk::{TokenZK as Token, TokenZK};
use aiken_lang::ast::Span;
use chumsky::combinator::To;
use crate::zk_examples::{InputVisibility, ZkExample};

pub struct AikenZkCompiler;

impl AikenZkCompiler {


    fn find_offchain_token(tokens: Vec<(Token, Span)>) -> Some<&'static (Token, Span)> {
        tokens.iter().find(|(token, span)| matches!(token, Token::Offchain {..}))
    }

    fn replace_keyword_with_function_call(aiken_src: &str, token: &Token, span: &Span) -> String {
        let mut aiken_zk_src = String::from(aiken_src);
        let public_identifiers = Self::extract_public_identifiers_from_token(token);

        let replacement = String::new();
        aiken_zk_src.replace_range(span.start..span.end, &replacement);
        aiken_zk_src
    }

    fn extract_identifier_from_token(token: &Token) -> String {
        match token {
            Token::Name {name} => name.clone(),
            Token::Int {value, ..} => value.clone(),
            _ => panic!("Not the expected kind of token")
        }
    }

    fn extract_public_identifiers_from_token(token: &TokenZK) {
        match token {
            Token::Offchain { example: ZkExample::Addition { lhs, rhs, res } } => {
                let public_inputs_identifiers: Vec<String> = [lhs, rhs, res].iter().fold(
                    vec![],
                    |mut acc, &input| match input.visibility.clone() {
                        Some(InputVisibility::Public) => {
                                acc.push(Self::extract_identifier_from_token(token));
                                acc
                            }
                        None => {
                            acc.push(Self::extract_identifier_from_token(token));
                            acc
                        }
                        _ => acc
                    },
                );
            }
            _ => panic!("Not implemented")
        }
    }

    pub fn apply_modifications_to_src_for_token(aiken_src: String, aiken_src_filename: String) -> String {
        let LexInfo { tokens, .. } = Lexer::new().run(&aiken_src).unwrap();
        let (token, span) = Self::find_offchain_token(tokens).unwrap();
        let circom_component_src = ComponentCreator::from_token(token).create();

        let mut circom_compiler = CircomCompiler::from(circom_component_src);
        let circom_src_filename_with_extension = aiken_src_filename + ".circom";
        circom_compiler.save_into_file(circom_src_filename_with_extension.clone()).unwrap();
        circom_compiler.create_verification_key(circom_src_filename_with_extension).unwrap();

        // Leer vk
        // Comprimir los puntos de curva
        // crear .ak modificado con el contenido de la vk



        Ok("".to_string())
    }
}