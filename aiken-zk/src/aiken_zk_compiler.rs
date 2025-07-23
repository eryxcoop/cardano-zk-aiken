use crate::circom_circuit;
use crate::circom_circuit::CircomCircuit;
use crate::component_creator::ComponentCreator;
use crate::compressed_groth16_proof_bls12_381_aiken_presenter::CompressedGroth16ProofBls12_381AikenPresenter;
use crate::lexer::{LexInfo, Lexer};
use crate::token_zk::{TokenZK as Token, TokenZK};
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
    pub fn generate_mesh_js_zk_redeemer_library(
        circom_path: &str,
        verification_key_path: &str,
        inputs_path: &str,
        output_path: &str,
    ) {
        let mut zk_redeemer = r#"import {MConStr} from "@meshsdk/common";
import {Data, mConStr0} from "@meshsdk/core";

type Proof = MConStr<any, string[]>;

type ZKRedeemer = MConStr<any, Data[] | Proof[]>;

function mProof(piA: string, piB: string, piC: string): Proof {
    if (piA.length != 96 || piB.length != 192 || piC.length != 96) {
        throw new Error("Wrong proof");
    }

    return mConStr0([piA, piB, piC]);
}

export function mZKRedeemer(redeemer: Data): ZKRedeemer {
    return mConStr0([redeemer, proofs()]);
}

function proofs(): Proof[] {
    return [
"#
        .to_string()
            + "        mProof(\n";
        let circuit = CircomCircuit::from(circom_path.to_string());
        let proof = circuit.generate_groth16_proof(verification_key_path, inputs_path);
        zk_redeemer += &("            \"".to_string() + proof.piA_as_byte_string() + "\",\n");
        zk_redeemer += &("            \"".to_string() + proof.piB_as_byte_string() + "\",\n");
        zk_redeemer += &("            \"".to_string() + proof.piC_as_byte_string() + "\",\n");
        fs::write(output_path, zk_redeemer).expect("output file write failed");
    }
}

impl AikenZkCompiler {
    fn find_offchain_token(tokens: Vec<(Token, Span)>) -> (Token, Span) {
        tokens
            .iter()
            .find(|(token, _span)| matches!(token, Token::Offchain { .. }))
            .unwrap()
            .clone()
    }

    fn replace_keyword_with_function_call(aiken_src: &str, token: &Token, span: &Span) -> String {
        let mut aiken_zk_src = String::from(aiken_src);
        let public_identifiers = Self::extract_public_identifiers_from_token(token);
        let replacement = format!(
            "zk_verify_or_fail(redeemer, [{}])",
            public_identifiers.join(", ")
        );
        aiken_zk_src.replace_range(span.start..span.end, &replacement);
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

    pub fn apply_modifications_to_src_for_token(
        aiken_src: String,
        aiken_src_filename: String,
        random_seeds: (&str, &str),
    ) -> String {
        let LexInfo { tokens, .. } = Lexer::new().run(&aiken_src).unwrap();
        let (token, span) = Self::find_offchain_token(tokens);
        let circom_component_src = ComponentCreator::from_token(token.clone()).create();

        let circom_src_filename_with_extension = aiken_src_filename + ".circom";
        fs::write(&circom_src_filename_with_extension, circom_component_src).unwrap();
        let mut circom_compiler = CircomCircuit::from(circom_src_filename_with_extension.clone());

        circom_compiler
            .generate_verification_key(random_seeds)
            .unwrap();

        let vk_compressed_data = Self::extract_vk_compressed_data().unwrap();

        let mut aiken_zk_src = Self::replace_keyword_with_function_call(&aiken_src, &token, &span);
        aiken_zk_src = Self::prepend_imports(&aiken_zk_src);
        aiken_zk_src =
            Self::append_verify_function_declaration(aiken_zk_src, &token, &vk_compressed_data);
        aiken_zk_src
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

    pub fn generate_aiken_proof(
        circom_path: &str,
        verification_key_path: &str,
        inputs_path: &str,
        output_path: &str,
    ) {
        let circom_compiler = CircomCircuit::from(circom_path.to_string());
        let proof = circom_compiler.generate_groth16_proof(verification_key_path, inputs_path);

        let aiken_presenter = CompressedGroth16ProofBls12_381AikenPresenter::new(proof);

        let aiken_proof = aiken_presenter.present();
        fs::write(output_path, aiken_proof).expect("failed to create output file");
    }
}
