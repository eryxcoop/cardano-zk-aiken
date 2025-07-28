use crate::aiken_zk_compiler::AikenZkCompiler;
use crate::subcommand::Subcommand;
use clap::{ArgMatches, Command};
use std::path::{Path, PathBuf};

pub struct ProveCommand {}

impl Subcommand for ProveCommand {
    const SUBCOMMAND_NAME: &'static str = "prove";

    fn create_subcommand() -> Command {
        let circom_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_CIRCOM_ARG_NAME);
        let verification_key_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_VK_ARG_NAME);
        let inputs_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_INPUT_ARG_NAME);
        let output_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_OUTPUT_ARG_NAME);

        Command::new(Self::SUBCOMMAND_NAME)
            .arg(circom_path)
            .arg(verification_key_path)
            .arg(inputs_path)
            .arg(output_path)
    }

    fn for_name(name: &str) -> bool {
        Self::SUBCOMMAND_NAME.to_string() == name.to_string()
    }

    fn evaluate(&self, matches: &ArgMatches) {
        let (circom_path, verification_key_path, inputs_path, output_path) =
            Self::get_prove_arguments(matches);
        Self::execute_prove_command(circom_path, verification_key_path, inputs_path, output_path);
    }
}

impl ProveCommand {
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
