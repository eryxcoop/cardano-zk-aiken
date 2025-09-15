use crate::cli::subcommand::Subcommand;
use clap::{ArgMatches, Command};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct NewCommand;

impl Subcommand for NewCommand {
    const SUBCOMMAND_NAME: &'static str = "new";

    fn create_subcommand() -> Command {
        let project_name = Self::create_required_argument_with_id(Self::NEW_COMMAND_PROJECT_NAME_ARG_NAME);

        Command::new(Self::SUBCOMMAND_NAME).arg(project_name.clone())
    }

    fn for_name(name: &str) -> bool {
        name.to_string() == Self::SUBCOMMAND_NAME.to_string()
    }

    fn evaluate(&self, matches: &ArgMatches) {
        let project_name = Self::get_arguments(matches);
        Self::execute_command(project_name);
    }
}

impl NewCommand {
    const NEW_COMMAND_PROJECT_NAME_ARG_NAME: &'static str = "project_name";

    fn execute_command(project_name: &PathBuf) {

        // Create a directory called [project_name]
        // Copy the static template tree structure in the directory
        // Run [npm i | yarn i | ...] over the desired subdirectories
        // Check for the global installation of aiken, circom, snarkjs. Warn the user if they're not installed


        let new_dir_name = fs::read_to_string(project_name).unwrap();
        Self::create_new_dir_if_non_existent(&new_dir_name)
            .expect("Unable to create working directory");

    }

    fn get_arguments(subcommand_matches: &ArgMatches) -> &PathBuf {
        Self::get_argument_value(subcommand_matches, Self::NEW_COMMAND_PROJECT_NAME_ARG_NAME)
    }

    pub fn create_new_dir_if_non_existent(dir_name: &str) -> Result<(), Error>{
        fs::create_dir(dir_name)
    }
}