use crate::aiken_zk_compiler::AikenZkCompiler;
use crate::create_validators_dir_lazy;
use clap::{Arg, ArgMatches, Command, value_parser};
use std::fs;
use std::path::{Path, PathBuf};
use aiken_lang::tipo::Type::Tuple;

pub struct CommandLineInterface;
impl CommandLineInterface {
    const BUILD_COMMAND_NAME: &'static str = "build";
    const BUILD_COMMAND_SOURCE_ARG_NAME: &'static str = "source_path";
    const BUILD_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_path";

    const PROVE_COMMAND_NAME: &'static str = "prove";
    const PROVE_COMMAND_CIRCOM_ARG_NAME: &'static str = "circom_path";
    const PROVE_COMMAND_VK_ARG_NAME: &'static str = "verification_key_path";
    const PROVE_COMMAND_INPUT_ARG_NAME: &'static str = "inputs_path";
    const PROVE_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_proof_path";

    pub fn parse_inputs_and_execute_command() {
        let main_command = Self::create_main_command();
        let main_command_matches = main_command.get_matches();

        let subcommands: Vec<(&str, fn(&ArgMatches))> = vec![
            ("build", |matches| {
                let (source_path, output_path) = Self::get_build_arguments(matches);
                create_validators_dir_lazy();
                Self::execute_build_command(source_path, output_path);
            }),
            ("prove", |_: &ArgMatches| {})
        ];
        let subcommand = main_command_matches.subcommand();
        if subcommand.is_some() {
            let (name, matches) = subcommand.unwrap();
            if name == subcommands[0].0 {
                subcommands[0].1(matches);
            } else if name == subcommands[1].0 {
                let (circom_path, verification_key_path, inputs_path, output_path) =
                    Self::get_prove_arguments(matches);
                Self::execute_prove_command(
                    circom_path,
                    verification_key_path,
                    inputs_path,
                    output_path,
                );
            }
        } else {
        }
    }

    fn execute_prove_command(
        circom_path: &Path,
        verification_key_path: &Path,
        inputs_path: &Path,
        output_path: &Path,
    ) {
        AikenZkCompiler::generate_aiken_proof(
            circom_path.to_str().unwrap(),
            verification_key_path.to_str().unwrap(),
            inputs_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
        );
    }

    fn get_prove_arguments(
        subcommand_matches: &ArgMatches,
    ) -> (&PathBuf, &PathBuf, &PathBuf, &PathBuf) {
        let circom_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_CIRCOM_ARG_NAME);
        let verification_key_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_VK_ARG_NAME);
        let inputs_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_INPUT_ARG_NAME);
        let output_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_OUTPUT_ARG_NAME);
        (circom_path, verification_key_path, inputs_path, output_path)
    }

    fn execute_build_command(source_path: &PathBuf, output_path: &PathBuf) {
        let source_offchain_aiken = fs::read_to_string(source_path).unwrap();

        let output_zk_aiken = AikenZkCompiler::apply_modifications_to_src_for_token(
            source_offchain_aiken,
            "output".to_string(),
            ("random1", "random2"),
        );

        fs::write(output_path, output_zk_aiken).expect("output file write failed");
    }

    fn get_build_arguments(subcommand_matches: &ArgMatches) -> (&PathBuf, &PathBuf) {
        let source_path =
            Self::get_argument_value(subcommand_matches, Self::BUILD_COMMAND_SOURCE_ARG_NAME);
        let output_path =
            Self::get_argument_value(subcommand_matches, Self::BUILD_COMMAND_OUTPUT_ARG_NAME);
        (source_path, output_path)
    }

    fn create_main_command() -> Command {
        Command::new("aiken-zk")
            .subcommand_required(true)
            .subcommand(Self::create_build_subcommand())
            .subcommand(Self::create_prove_subcommand())
    }

    fn get_argument_value<'a>(
        subcommand_matches: &'a ArgMatches,
        argument_id: &str,
    ) -> &'a PathBuf {
        subcommand_matches
            .get_one::<PathBuf>(argument_id)
            .expect("Value for command not found")
    }

    fn create_build_subcommand() -> Command {
        let input = Self::create_required_argument_with_id(Self::BUILD_COMMAND_SOURCE_ARG_NAME);
        let output = Self::create_required_argument_with_id(Self::BUILD_COMMAND_OUTPUT_ARG_NAME);

        Command::new(Self::BUILD_COMMAND_NAME)
            .arg(input.clone())
            .arg(output.clone())
    }

    fn create_prove_subcommand() -> Command {
        let circom_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_CIRCOM_ARG_NAME);
        let verification_key_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_VK_ARG_NAME);
        let inputs_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_INPUT_ARG_NAME);
        let output_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_OUTPUT_ARG_NAME);

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
