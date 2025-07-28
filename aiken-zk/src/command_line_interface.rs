use crate::aiken_zk_compiler::AikenZkCompiler;
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::{Path, PathBuf};
use crate::build_command::BuildCommand;

pub trait Subcommand {
    fn create_subcommand() -> Command;

    fn for_name(name: &str) -> bool;

    fn evaluate(&self, matches: &ArgMatches);

    fn create_required_argument_with_id(id: &'static str) -> Arg {
        Arg::new(id)
            .required(true)
            .value_parser(value_parser!(PathBuf))
    }

    fn get_argument_value<'a>(
        subcommand_matches: &'a ArgMatches,
        argument_id: &str,
    ) -> &'a PathBuf {
        subcommand_matches
            .get_one::<PathBuf>(argument_id)
            .expect("Value for command not found")
    }
}

struct ProveCommand {}

impl Subcommand for ProveCommand {
    fn create_subcommand() -> Command {
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

    fn for_name(name: &str) -> bool {
        "prove".to_string() == name.to_string()
    }

    fn evaluate(&self, matches: &ArgMatches) {
        let (circom_path, verification_key_path, inputs_path, output_path) =
            Self::get_prove_arguments(matches);
        Self::execute_prove_command(circom_path, verification_key_path, inputs_path, output_path);
    }
}

impl ProveCommand {
    const PROVE_COMMAND_NAME: &'static str = "prove";
    const PROVE_COMMAND_CIRCOM_ARG_NAME: &'static str = "circom_path";
    const PROVE_COMMAND_VK_ARG_NAME: &'static str = "verification_key_path";
    const PROVE_COMMAND_INPUT_ARG_NAME: &'static str = "inputs_path";
    const PROVE_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_proof_path";

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
}

pub struct CommandLineInterface;

macro_rules! execute_subcommand {
    ( $subcommand: ident, $a_command: ident, $( $others_commands:ident ),* ) => {
        {
            let (match_name, matches) = $subcommand;

            if ($a_command::for_name(match_name)) {
                    let command = $a_command {};
                    command.evaluate(matches);
            }
            $(
                else if ($others_commands::for_name(match_name)) {
                    let command = $others_commands {};
                    command.evaluate(matches);
                }
            )*
            else {
                panic!("Unknown command: {}", stringify!(match_name));
            }
        };
    };
}
impl CommandLineInterface {
    pub fn parse_inputs_and_execute_command() {
        let main_command = Self::create_main_command();
        let main_command_matches = main_command.get_matches();

        match main_command_matches.subcommand() {
            Some(subcommand) => {
                execute_subcommand!(subcommand, BuildCommand, ProveCommand);
            }
            None => {
                panic!("No command given");
            }
        }
    }

    fn create_main_command() -> Command {
        Command::new("aiken-zk")
            .subcommand_required(true)
            .subcommand(BuildCommand::create_subcommand())
            .subcommand(ProveCommand::create_subcommand())
    }
}
