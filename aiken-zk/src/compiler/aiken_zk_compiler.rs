use crate::circom_circuit::CircomCircuit;
use crate::component_creator::ComponentCreator;
use crate::compiler::lexer::{LexInfo, Lexer};
use crate::compiler::token_zk::{TokenZK as Token, TokenZK};
use crate::zk_examples::{InputVisibility, InputZK, ZkExample};
use aiken_lang::ast::Span;
use serde::Deserialize;
use std::fs;
use std::io::Error;
use std::process::Command;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Groth16CompressedData {
    pub vk_alpha_1: String,
    pub vk_beta_2: String,
    pub vk_gamma_2: String,
    pub vk_delta_2: String,
    pub IC: Vec<String>,
}

pub struct AikenZkCompiler;

impl AikenZkCompiler {
    pub fn apply_modifications_to_src_for_token(
        aiken_src: String,
        aiken_src_filename: String,
        random_seeds: (&str, &str),
    ) -> String {
        let (offchain_token, offchain_token_span) = Self::detect_code_to_replace(&aiken_src);
        match &offchain_token {
            TokenZK::Offchain {
                example:ZkExample::CustomCircom {
                    path,
                    public_input,
                }
            } => {
                let circom_circuit = CircomCircuit::from(path.clone());
                circom_circuit
                    .generate_verification_key(random_seeds)
                    .unwrap();
                // Replace offchain with groth16 verifier
                let vk_compressed_data = Self::extract_vk_compressed_data().unwrap();
                let public_input_identifiers: Vec<String> = public_input.iter()
                    .map(|token| {
                        Self::extract_identifier_from_token(token)
                    })
                    .collect();

                let mut aiken_zk_src = String::from(&aiken_src);
                let replacement = format!(
                    "zk_verify_or_fail(redeemer, [{}])",
                    public_input_identifiers.join(", ")
                );
                aiken_zk_src.replace_range(
                    offchain_token_span.start..offchain_token_span.end,
                    &replacement,
                );
                aiken_zk_src = Self::prepend_imports(&aiken_zk_src);
                aiken_zk_src = Self::append_verify_function_declaration(
                    aiken_zk_src,
                    &offchain_token,
                    &vk_compressed_data,
                );
                aiken_zk_src
            }
            _ => {
                Self::yyy(&aiken_src, aiken_src_filename, random_seeds, &offchain_token, &offchain_token_span)
            }
        }
        }

    fn yyy(aiken_src: &String, aiken_src_filename: String, random_seeds: (&str, &str), offchain_token: &TokenZK, offchain_token_span: &Span) -> String {
        Self::output_offchain_circuit_and_reference(aiken_src_filename, random_seeds, &offchain_token);
        Self::output_aiken_code(&aiken_src, &offchain_token, &offchain_token_span)
    }

    fn detect_code_to_replace(aiken_src: &String) -> (TokenZK, Span) {
        // Detect offchain token
        let LexInfo { tokens, .. } = Lexer::new().run(&aiken_src).unwrap();
        let (offchain_token, offchain_token_span) = Self::find_offchain_token(tokens);
        (offchain_token, offchain_token_span)
    }

    fn output_offchain_circuit_and_reference(
        aiken_src_filename: String,
        random_seeds: (&str, &str),
        offchain_token: &TokenZK,
    ) {
        // Create circom circuit source code for offchain token
        let circom_component_src = ComponentCreator::from_token(offchain_token.clone()).create();
        let circom_src_filename_with_extension = aiken_src_filename + ".circom";
        fs::write(&circom_src_filename_with_extension, circom_component_src).unwrap();

        // Create verification key for circom circuit
        let circom_circuit = CircomCircuit::from(circom_src_filename_with_extension.clone());
        circom_circuit
            .generate_verification_key(random_seeds)
            .unwrap();
    }

    fn output_aiken_code(
        aiken_src: &String,
        offchain_token: &TokenZK,
        offchain_token_span: &Span,
    ) -> String {
        // Replace offchain with groth16 verifier
        let vk_compressed_data = Self::extract_vk_compressed_data().unwrap();
        let mut aiken_zk_src = Self::replace_keyword_with_function_call(
            &aiken_src,
            &offchain_token,
            &offchain_token_span,
        );
        aiken_zk_src = Self::prepend_imports(&aiken_zk_src);
        aiken_zk_src = Self::append_verify_function_declaration(
            aiken_zk_src,
            &offchain_token,
            &vk_compressed_data,
        );
        aiken_zk_src
    }

    fn find_offchain_token(tokens: Vec<(Token, Span)>) -> (Token, Span) {
        tokens
            .iter()
            .find(|(token, _span)| matches!(token, Token::Offchain { .. }))
            .unwrap()
            .clone()
    }

    fn replace_keyword_with_function_call(
        aiken_src: &str,
        token: &Token,
        offchain_token_span: &Span,
    ) -> String {
        let mut aiken_zk_src = String::from(aiken_src);
        let public_identifiers = Self::extract_public_identifiers_from_token(token);
        let replacement = format!(
            "zk_verify_or_fail(redeemer, [{}])",
            public_identifiers.join(", ")
        );
        aiken_zk_src.replace_range(
            offchain_token_span.start..offchain_token_span.end,
            &replacement,
        );
        aiken_zk_src
    }

    fn extract_identifier_from_token(token: &Token) -> String {
        match token {
            Token::Name { name } => name.clone(),
            Token::Int { value, .. } => value.clone(),
            _ => panic!("Not the expected kind of token"),
        }
    }

    fn extract_public_identifiers_from_token(token: &TokenZK) -> Vec<String> {
        match token {
            Token::Offchain {
                example: ZkExample::Addition { lhs, rhs, res },
            } => [lhs, rhs, res].iter().fold(vec![], |acc, &input| {
                Self::extract_visibility_from_input(acc, &input)
            }),

            Token::Offchain {
                example: ZkExample::Subtraction { lhs, rhs, res },
            } => [lhs, rhs, res].iter().fold(vec![], |acc, &input| {
                Self::extract_visibility_from_input(acc, &input)
            }),

            Token::Offchain {
                example: ZkExample::Multiplication { lhs, rhs, res },
            } => [lhs, rhs, res].iter().fold(vec![], |acc, &input| {
                Self::extract_visibility_from_input(acc, &input)
            }),

            Token::Offchain {
                example:
                    ZkExample::Fibonacci {
                        fib_0, fib_1, res, ..
                    },
            } => [fib_0, fib_1, res].iter().fold(vec![], |acc, &input| {
                Self::extract_visibility_from_input(acc, &input)
            }),

            Token::Offchain {
                example:
                    ZkExample::If {
                        condition,
                        assigned,
                        true_branch,
                        false_branch,
                    },
            } => [condition, assigned, true_branch, false_branch]
                .iter()
                .fold(vec![], |acc, &input| {
                    Self::extract_visibility_from_input(acc, &input)
                }),

            Token::Offchain {
                example: ZkExample::AssertEq { lhs, rhs },
            } => [lhs, rhs].iter().fold(vec![], |acc, &input| {
                Self::extract_visibility_from_input(acc, &input)
            }),

            _ => panic!("Not implemented"),
        }
    }

    fn extract_visibility_from_input(mut acc: Vec<String>, input: &&InputZK) -> Vec<String> {
        match input.visibility.clone() {
            Some(InputVisibility::Private) => acc,
            _ => {
                acc.push(Self::extract_identifier_from_token(&input.token));
                acc
            }
        }
    }

    fn append_verify_function_declaration(
        aiken_zk_src: String,
        token: &Token,
        vk_compressed_data: &Groth16CompressedData,
    ) -> String {
        let public_input_count = Self::extract_public_identifiers_from_token(token).len();
        let full_verify_function_declaration =
            Self::create_verify_function_declaration_from(vk_compressed_data, public_input_count);
        aiken_zk_src + &full_verify_function_declaration
    }

    fn extract_vk_compressed_data() -> Result<Groth16CompressedData, Error> {
        let output = Command::new("node")
            .arg("curve_compress/compressedVerificationKey.js")
            .arg("build/verification_key.json")
            .output()?;

        let stdout = String::from_utf8(output.stdout).unwrap();
        let compressed_vk: Groth16CompressedData = serde_json::from_str(&stdout)?;

        Ok(compressed_vk)
    }

    fn create_verify_function_declaration_from(
        vk_compressed_data: &Groth16CompressedData,
        public_input_count: usize,
    ) -> String {
        let formatted_ic = vk_compressed_data
            .IC
            .iter()
            .map(|h| format!("                #\"{h}\""))
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            r#"
    fn zk_verify_or_fail(
        zk_redeemer: ZK<Redeemer>,
        public_inputs: List<Int>
    ) -> ZK<Redeemer> {{

        let vk: SnarkVerificationKey =
            SnarkVerificationKey {{
                nPublic: {},
                vkAlpha: #"{vkAlpha}",
                vkBeta: #"{vkBeta}",
                vkGamma: #"{vkGamma}",
                vkDelta: #"{vkDelta}",
                vkAlphaBeta: [],
                vkIC: [
{formatted_ic},
                ],
            }}

        expect Some(proof) = list.head(zk_redeemer.proofs)

        if !groth_verify(vk, proof, public_inputs) {{
          fail
          Void
        }} else {{
          Void
        }}

        expect Some(proofs) = list.tail(zk_redeemer.proofs)
        ZK {{ redeemer: zk_redeemer.redeemer, proofs }}
    }}"#,
            public_input_count,
            vkAlpha = vk_compressed_data.vk_alpha_1,
            vkBeta = vk_compressed_data.vk_beta_2,
            vkGamma = vk_compressed_data.vk_gamma_2,
            vkDelta = vk_compressed_data.vk_delta_2,
            formatted_ic = formatted_ic,
        )
    }

    fn prepend_imports(aiken_src: &str) -> String {
        "use aiken/collection/list\nuse ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}\n".to_string() + aiken_src
    }
}
