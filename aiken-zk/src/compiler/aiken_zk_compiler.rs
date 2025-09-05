use crate::circom_circuit::CircomCircuit;
use crate::compiler::BUILD_DIR;
use crate::compiler::lexer::{LexInfo, Lexer};
use crate::compiler::token_zk::{TokenZK as Token, TokenZK};
use crate::component_creator::ComponentCreator;
use crate::zk_examples::{InputVisibility, InputZK, TokenWithCardinality, ZkExample};
use aiken_lang::ast::Span;
use serde::Deserialize;
use serde_json::Value;
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
                example:
                    ZkExample::CustomCircom {
                        path,
                        public_inputs: public_input,
                    },
            } => Self::apply_modifications_to_src_for_custom_token(
                &aiken_src,
                random_seeds,
                offchain_token_span,
                path,
                public_input,
            ),
            TokenZK::Offchain { example } => Self::apply_modifications_to_src_for_example_token(
                &aiken_src,
                aiken_src_filename,
                random_seeds,
                &offchain_token,
                offchain_token_span,
                example,
            ),
            _ => panic!(""),
        }
    }

    fn apply_modifications_to_src_for_custom_token(
        aiken_src: &str,
        random_seeds: (&str, &str),
        offchain_token_span: Span,
        path: &str,
        public_inputs: &Vec<Box<TokenWithCardinality>>,
    ) -> String {
        let output_path = BUILD_DIR;
        let circom_circuit = CircomCircuit::from(path.to_string());
        let circuit_name = circom_circuit.filename();

        circom_circuit
            .generate_verification_key(random_seeds)
            .unwrap();

        let vk_compressed_data = Self::extract_vk_compressed_data().unwrap();
        let public_input_identifiers: Vec<String> = public_inputs
            .iter()
            .map(|token| {
                let token = TokenWithCardinality::extract_single(token);
                Self::extract_identifier_from_token(&token.unwrap())
            })
            .collect();

        Self::assert_equal_number_of_public_inputs(
            output_path,
            &circuit_name,
            circom_circuit,
            &public_input_identifiers,
        );

        let mut aiken_zk_src =
            Self::replace_range_of_offchain_keyword_by_verification_function_call_using_single_parameters(
                aiken_src,
                offchain_token_span,
                &public_input_identifiers,
            );
        let public_input_count = public_input_identifiers.len();
        aiken_zk_src = Self::prepend_imports(&aiken_zk_src);

        Self::append_verify_function_declaration(
            aiken_zk_src,
            &vk_compressed_data,
            public_input_count,
        )
    }

    fn replace_range_of_offchain_keyword_by_verification_function_call(
        aiken_src: &str,
        offchain_token_span: Span,
        public_input_identifiers: &Vec<String>,
        example: &ZkExample,
    ) -> String {
        let mut aiken_zk_src = aiken_src.to_string();
        let public_input_identifiers_wrapped_with_list_characteristics: Vec<String> = match example
        {
            ZkExample::Sha256 {
                n_bits: _,
                r#in: _,
                out: _,
            } => public_input_identifiers
                .iter()
                .map(|identifier| "Many(".to_string() + identifier + ")")
                .collect(),
            ZkExample::Poseidon {
                n_inputs: _,
                r#in: _,
                out: _,
            } => public_input_identifiers
                .iter()
                .enumerate()
                .map(|(i,identifier)| {
                    match i {
                        0 => "Many(".to_string() + identifier + ")",
                        1 => "Single(".to_string() + identifier + ")",
                        _ => panic!("Invalid MerkleTreeChecker identifier")
                    }
                })
                .collect(),
            ZkExample::MerkleTreeChecker {
                levels: _,
                leaf: _,
                root: _,
                path_elements: _,
                path_indices: _,
            } => public_input_identifiers
                .iter()
                .enumerate()
                .map(|(i,identifier)| {
                    match i {
                        0..=1 => "Single(".to_string() + identifier + ")",
                        2..=3 => "Many(".to_string() + identifier + ")",
                        _ => panic!("Invalid MerkleTreeChecker identifier")
                    }
                })
                .collect(),
            _ => public_input_identifiers
                .iter()
                .map(|identifier| "Single(".to_string() + identifier + ")")
                .collect(),
        };

        let replacement = format!(
            "zk_verify_or_fail(redeemer, [{}])",
            public_input_identifiers_wrapped_with_list_characteristics.join(", ")
        );
        aiken_zk_src.replace_range(
            offchain_token_span.start..offchain_token_span.end,
            &replacement,
        );
        aiken_zk_src
    }

    fn replace_range_of_offchain_keyword_by_verification_function_call_using_single_parameters(
        aiken_src: &str,
        offchain_token_span: Span,
        public_input_identifiers: &Vec<String>,
    ) -> String {
        let mut aiken_zk_src = aiken_src.to_string();
        let public_input_identifiers_wrapped_with_list_characteristics: Vec<String> =
            public_input_identifiers
                .iter()
                .map(|identifier| "Single(".to_string() + identifier + ")")
                .collect();

        let replacement = format!(
            "zk_verify_or_fail(redeemer, [{}])",
            public_input_identifiers_wrapped_with_list_characteristics.join(", ")
        );
        aiken_zk_src.replace_range(
            offchain_token_span.start..offchain_token_span.end,
            &replacement,
        );
        aiken_zk_src
    }

    fn assert_equal_number_of_public_inputs(
        output_path: &str,
        circuit_name: &str,
        circom_circuit: CircomCircuit,
        public_input_identifiers: &Vec<String>,
    ) {
        let r1cs_path = format!("{}{}.r1cs", output_path, circuit_name);
        let r1cs_json_path = format!("{}.json", r1cs_path);
        circom_circuit.export_r1cs_to_json(&r1cs_path, &r1cs_json_path);
        let r1cs_json_str = fs::read_to_string(&r1cs_json_path).unwrap();
        let json: Value = serde_json::from_str(&r1cs_json_str).unwrap();
        let public_inputs_amount = json["nPubInputs"].as_u64().unwrap();
        assert_eq!(
            public_input_identifiers.len(),
            public_inputs_amount as usize,
            "Amount of public inputs doesnt match"
        );
    }

    fn apply_modifications_to_src_for_example_token(
        aiken_src: &String,
        aiken_src_filename: String,
        random_seeds: (&str, &str),
        offchain_token: &TokenZK,
        offchain_token_span: Span,
        example: &ZkExample,
    ) -> String {
        Self::output_offchain_circuit_and_reference(
            aiken_src_filename,
            random_seeds,
            &offchain_token,
        );
        Self::output_aiken_code(&aiken_src, &offchain_token, offchain_token_span, example)
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
        offchain_token_span: Span,
        example: &ZkExample,
    ) -> String {
        // Replace offchain with groth16 verifier
        let vk_compressed_data = Self::extract_vk_compressed_data().unwrap();
        let mut aiken_zk_src = Self::replace_keyword_with_function_call(
            &aiken_src,
            &offchain_token,
            offchain_token_span,
            example,
        );
        let public_input_count = Self::extract_public_identifiers_from_token(offchain_token).len();
        aiken_zk_src = Self::prepend_imports(&aiken_zk_src);
        aiken_zk_src = Self::append_verify_function_declaration(
            aiken_zk_src,
            &vk_compressed_data,
            public_input_count,
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
        offchain_token_span: Span,
        example: &ZkExample,
    ) -> String {
        let public_input_identifiers = Self::extract_public_identifiers_from_token(token);
        Self::replace_range_of_offchain_keyword_by_verification_function_call(
            aiken_src,
            offchain_token_span,
            &public_input_identifiers,
            example,
        )
    }

    fn extract_identifier_from_token(token: &TokenZK) -> String {
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
            } => [lhs, rhs, res]
                .into_iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example: ZkExample::Subtraction { lhs, rhs, res },
            } => [lhs, rhs, res]
                .into_iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example: ZkExample::Multiplication { lhs, rhs, res },
            } => [lhs, rhs, res]
                .into_iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example:
                    ZkExample::Fibonacci {
                        fib_0, fib_1, res, ..
                    },
            } => [fib_0, fib_1, res]
                .into_iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example:
                    ZkExample::If {
                        condition,
                        assigned,
                        true_branch,
                        false_branch,
                    },
            } => [condition, assigned, true_branch, false_branch]
                .into_iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example: ZkExample::AssertEq { lhs, rhs },
            } => [lhs, rhs]
                .into_iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example: ZkExample::Sha256 { r#in, out, .. },
            } => [r#in, out]
                .iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example: ZkExample::Poseidon { r#in, out, .. },
            } => [r#in, out]
                .iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            Token::Offchain {
                example: ZkExample::MerkleTreeChecker { leaf, root, path_elements, path_indices, .. },
            } => [leaf, root, path_elements, path_indices]
                .iter()
                .filter_map(|input| Self::extract_visibility_from_input(&input))
                .collect(),

            _ => panic!("Not implemented"),
        }
    }

    fn extract_visibility_from_input(input: &InputZK) -> Option<String> {
        match input.visibility.clone() {
            InputVisibility::Private => None,
            _ => {
                let token = TokenWithCardinality::extract_single(&input.token.clone().unwrap());
                Some(Self::extract_identifier_from_token(&token.unwrap()))
            }
        }
    }

    fn append_verify_function_declaration(
        aiken_zk_src: String,
        vk_compressed_data: &Groth16CompressedData,
        public_input_count: usize,
    ) -> String {
        let full_verify_function_declaration =
            Self::create_verify_function_declaration_from(vk_compressed_data, public_input_count);
        aiken_zk_src + &full_verify_function_declaration
    }

    fn extract_vk_compressed_data() -> Result<Groth16CompressedData, Error> {
        let output = Command::new("node")
            .arg("curve_compress/compressedVerificationKey.js")
            .arg(BUILD_DIR.to_string() + "verification_key.json")
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
    type ZKInputType {{
        Single(Int)
        Many(List<Int>)
    }}

    fn zk_verify_or_fail(
        zk_redeemer: ZK<Redeemer>,
        public_inputs: List<ZKInputType>
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

        let flattened_public_inputs: List<Int> = list.flat_map(public_inputs, fn(item) {{
            when item is {{
              Single(x) -> [x]
              Many(xs) -> xs
            }}
        }})

        if !groth_verify(vk, proof, flattened_public_inputs) {{
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
