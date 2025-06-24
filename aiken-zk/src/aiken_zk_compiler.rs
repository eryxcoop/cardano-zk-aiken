use std::io::Error;
use std::process::Command;
use crate::circom_compiler::CircomCompiler;
use crate::component_creator::ComponentCreator;
use crate::lexer::{LexInfo, Lexer};
use crate::token_zk::{TokenZK as Token, TokenZK};
use crate::zk_examples::{InputVisibility, ZkExample};
use aiken_lang::ast::Span;

struct Groth16CompressedData {
    vk_alpha: String,
    vk_beta: String,
    vk_gamma: String,
    vk_delta: String,
    vk_ic: Vec<String>,
}

pub struct AikenZkCompiler;
impl AikenZkCompiler {
    fn find_offchain_token(tokens: Vec<(Token, Span)>) -> (Token, Span) {
        tokens.iter().find(|(token, _span)| matches!(token, Token::Offchain {..})).unwrap().clone()
    }

    fn replace_keyword_with_function_call(aiken_src: &str, token: &Token, span: &Span) -> String {
        let mut aiken_zk_src = String::from(aiken_src);
        let public_identifiers = Self::extract_public_identifiers_from_token(token);
        let replacement = format!("zk_verify_or_fail(redeemer, [{}])", public_identifiers.join(", "));
        aiken_zk_src.replace_range(span.start..span.end, &replacement);
        aiken_zk_src
    }

    fn extract_identifier_from_token(token: &Token) -> String {
        match token {
            Token::Name { name } => name.clone(),
            Token::Int { value, .. } => value.clone(),
            _ => panic!("Not the expected kind of token")
        }
    }

    fn extract_public_identifiers_from_token(token: &TokenZK) -> Vec<String> {
        match token {
            Token::Offchain { example: ZkExample::Addition { lhs, rhs, res } } => {
                [lhs, rhs, res].iter().fold(
                    vec![],
                    |mut acc, &input| match input.visibility.clone() {
                        Some(InputVisibility::Private) => {
                            acc
                        }
                        _ => {
                            acc.push(Self::extract_identifier_from_token(&input.token));
                            acc
                        }
                    },
                )
            }
            _ => panic!("Not implemented")
        }
    }

    pub fn apply_modifications_to_src_for_token(aiken_src: String, aiken_src_filename: String, random_seeds: (&str, &str)) -> String {
        let LexInfo { tokens, .. } = Lexer::new().run(&aiken_src).unwrap();
        let (token, span) = Self::find_offchain_token(tokens);
        let circom_component_src = ComponentCreator::from_token(token.clone()).create();

        let mut circom_compiler = CircomCompiler::from(circom_component_src);
        let circom_src_filename_with_extension = aiken_src_filename + ".circom";
        circom_compiler.save_into_file(circom_src_filename_with_extension.clone()).unwrap();
        circom_compiler.create_verification_key(circom_src_filename_with_extension, random_seeds).unwrap();

        let vk_compressed_data = Self::extract_vk_compressed_data().unwrap();

        let mut aiken_zk_src = Self::replace_keyword_with_function_call(&aiken_src, &token, &span);
        aiken_zk_src = Self::append_verify_function_declaration(aiken_zk_src, &token, &vk_compressed_data);
        aiken_zk_src
    }

    fn append_verify_function_declaration(aiken_zk_src: String, token: &Token, vk_compressed_data: &Groth16CompressedData) -> String {
        let public_input_count = Self::extract_public_identifiers_from_token(token).len();
        let full_verify_function_declaration = Self::create_verify_function_declaration_from(vk_compressed_data, public_input_count);
        aiken_zk_src + &full_verify_function_declaration
    }

    fn extract_vk_compressed_data() -> Result<Groth16CompressedData, Error> {
        // Leer vk
        // Comprimir los puntos de curva
        println!("{:?}", Command::new("npx")
            .arg("tsx")
            .arg("curve_compress/index.js")
            .arg("build/verification_key.json")
            .output()?);

        Ok(Groth16CompressedData {
            vk_alpha: "85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5".to_string(),
            vk_beta: "b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f".to_string(),
            vk_gamma: "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8".to_string(),
            vk_delta: "a73193a83d15104c605596c6e366f22ba37c503a9313f2fbe9d83bb16f663644a859a0d73e68394b619063b1ebcf97710f296c0826476b5c9302d2b504e2ffe77827a715a066a230fe804ed06287b2442d65651a6488146302459955943492c7".to_string(),
            vk_ic: vec![
                "b42a4610c5c2722df0cae5b696d0e212dd41e471a5246217751ae313dceba2b4d25c1be296ee8e00454027b7c4a45208".to_string(),
                "87ef7b539de25c06493f7cd054a78da2819084b7027038d28b31fe88ce0b833f243723fbd9c4e502a3d0c2246aa69560".to_string(),
                "a680399022e0bd33fa72396626b4bfc5d1d42e6d9207f3bc64f9fd26a32e5d150ba63a7c28d61db724d362bb9cf96680".to_string(),
                "87ac4ff5d2863dd744e3ad397dfde8fe657c09c9c059e25ab8f37b85822eb8f34604d7ca2fe2622d1003ed258319bbf2".to_string(),
            ],
        })
    }

    fn create_verify_function_declaration_from(vk_compressed_data: &Groth16CompressedData, public_input_count: usize) -> String {
        let formatted_ic = vk_compressed_data.vk_ic
            .iter()
            .map(|h| format!("                #\"{h}\""))
            .collect::<Vec<_>>()
            .join(",\n");


        format!(r#"fn zk_verify_or_fail(
        zk_redeemer: ZK<Redeemer>,
        public_inputs: List<Int>
    ) -> ZK<Redeemer> {{
        let redeemer = zk_redeemer.redeemer

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

        if (!groth_verify(vk, proof, public_inputs)){{
            fail
        }}

        expect Some(proofs) = list.tail(zk_redeemer.proofs)
        let zk_redeemer = ZK {{ redeemer: zk_redeemer.redeemer, proofs }}
    }}"#,
                public_input_count,
                vkAlpha = vk_compressed_data.vk_alpha,
                vkBeta = vk_compressed_data.vk_beta,
                vkGamma = vk_compressed_data.vk_gamma,
                vkDelta = vk_compressed_data.vk_delta,
                formatted_ic = formatted_ic,
        )
    }
}