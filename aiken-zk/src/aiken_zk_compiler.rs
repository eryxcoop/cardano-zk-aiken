use crate::circom_compiler::CircomCompiler;
use crate::component_creator::ComponentCreator;
use crate::lexer::{LexInfo, Lexer};
use crate::token_zk::TokenZK as Token;
use aiken_lang::ast::Span;

pub struct AikenZkCompiler;

impl AikenZkCompiler {


    pub fn find_offchain_token(tokens: Vec<(Token, Span)>) -> Some<&'static (Token, Span)> {
        tokens.iter().find(|(token, span)| matches!(token, Token::Offchain {..}))
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