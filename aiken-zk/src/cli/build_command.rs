use crate::aiken_zk_compiler::AikenZkCompiler;
use crate::cli::subcommand::Subcommand;
use clap::{ArgMatches, Command};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct BuildCommand {}

impl Subcommand for BuildCommand {
    const SUBCOMMAND_NAME: &'static str = "build";
    
    fn create_subcommand() -> Command {
        let input = Self::create_required_argument_with_id(Self::BUILD_COMMAND_SOURCE_ARG_NAME);
        let output = Self::create_required_argument_with_id(Self::BUILD_COMMAND_OUTPUT_ARG_NAME);

        Command::new(Self::SUBCOMMAND_NAME)
            .arg(input.clone())
            .arg(output.clone())
    }

    fn for_name(name: &str) -> bool {
        name.to_string() == Self::SUBCOMMAND_NAME.to_string()
    }

    fn evaluate(&self, matches: &ArgMatches) {
        let (source_path, output_path) = Self::get_arguments(matches);
        Self::create_validators_dir_lazy();
        Self::execute_command(source_path, output_path);
    }
}

impl BuildCommand {
    const BUILD_COMMAND_SOURCE_ARG_NAME: &'static str = "source_path";
    const BUILD_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_path";

    fn execute_command(source_path: &PathBuf, output_path: &PathBuf) {
        let source_offchain_aiken = fs::read_to_string(source_path).unwrap();

        let output_zk_aiken = AikenZkCompiler::apply_modifications_to_src_for_token(
            source_offchain_aiken,
            "output".to_string(),
            ("random1", "random2"),
        );

        fs::write(output_path, output_zk_aiken).expect("output file write failed");
    }

    fn get_arguments(subcommand_matches: &ArgMatches) -> (&PathBuf, &PathBuf) {
        let source_path =
            Self::get_argument_value(subcommand_matches, Self::BUILD_COMMAND_SOURCE_ARG_NAME);
        let output_path =
            Self::get_argument_value(subcommand_matches, Self::BUILD_COMMAND_OUTPUT_ARG_NAME);
        (source_path, output_path)
    }

    pub fn create_validators_dir_lazy() {
        fs::create_dir("validators")
            .or_else(|error| {
                if error.kind() == ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Err(error)
                }
            })
            .expect("Couldnt create dir");
    }
}
