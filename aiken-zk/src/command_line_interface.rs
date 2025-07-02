use std::path::PathBuf;
use clap::{value_parser, Arg, ArgMatches, Command};
use crate::aiken_zk_compiler::AikenZkCompiler;
use std::fs;

pub struct CommandLineInterface;
impl CommandLineInterface {
    const BUILD_COMMAND_NAME: &'static str = "build";
    const BUILD_COMMAND_SOURCE_ARG_NAME: &'static str = "source_path";
    const BUILD_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_path";

    const PROVE_COMMAND_NAME: &'static str = "prove";
    // const PROVE_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_path";

    pub fn parse_inputs_and_execute_command() {
        let main_command = Self::create_main_command();

        let main_command_matches = main_command.get_matches();
        if let Some(subcommand_matches) = main_command_matches.subcommand_matches(Self::BUILD_COMMAND_NAME) {
            let source_path = Self::get_argument_value(&subcommand_matches, Self::BUILD_COMMAND_SOURCE_ARG_NAME);
            let output_path = Self::get_argument_value(&subcommand_matches, Self::BUILD_COMMAND_OUTPUT_ARG_NAME);

            let source_offchain_aiken = fs::read_to_string(source_path).unwrap();
            let output_zk_aiken = AikenZkCompiler::apply_modifications_to_src_for_token(
                source_offchain_aiken,
                "output".to_string(),
                ("random1", "random2"),
            );

            fs::write(output_path, output_zk_aiken).expect("output file write failed");
        } else if let Some(subcommand_matches) = main_command_matches.subcommand_matches(Self::PROVE_COMMAND_NAME) {
            let circom_path = Self::get_argument_value(&subcommand_matches, "circom_path");
            let verification_key_path = Self::get_argument_value(&subcommand_matches, "verification_key_path");
            let inputs_path = Self::get_argument_value(&subcommand_matches, "inputs_path");
            let output_path = Self::get_argument_value(&subcommand_matches, "output_proof_path");

            AikenZkCompiler::generate_aiken_proof(
                circom_path.to_str().unwrap(),
                verification_key_path.to_str().unwrap(),
                inputs_path.to_str().unwrap(),
                output_path.to_str().unwrap()
            );
        }
    }

    fn create_main_command() -> Command {
        Command::new("aiken-zk")
            .subcommand_required(true)
            .subcommand(Self::create_build_subcommand())
            .subcommand(Self::create_prove_subcommand())
    }

    fn get_argument_value<'a>(subcommand_matches: &'a ArgMatches, argument_id: &str) -> &'a PathBuf {
        subcommand_matches
            .get_one::<PathBuf>(argument_id)
            .expect("Value for command not found")
    }

    fn create_build_subcommand() -> Command {
        let input = Self::create_required_argument_with_id(Self::BUILD_COMMAND_SOURCE_ARG_NAME);
        let output = Self::create_required_argument_with_id(Self::BUILD_COMMAND_OUTPUT_ARG_NAME);

        Command::new(Self::BUILD_COMMAND_NAME).arg(input.clone()).arg(output.clone())
    }

    fn create_prove_subcommand() -> Command {
        let circom_path = Self::create_required_argument_with_id("circom_path");
        let verification_key_path = Self::create_required_argument_with_id("verification_key_path");
        let inputs_path = Self::create_required_argument_with_id("inputs_path");
        let output_path = Self::create_required_argument_with_id("output_proof_path");

        Command::new(Self::PROVE_COMMAND_NAME)
            .arg(circom_path)
            .arg(verification_key_path)
            .arg(inputs_path)
            .arg(output_path)
    }

    fn create_required_argument_with_id(id: &'static str) -> Arg {
        Arg::new(id)
            .required(true)
            .value_parser(value_parser!(PathBuf))
    }
}





