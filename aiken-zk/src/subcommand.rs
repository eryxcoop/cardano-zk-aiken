use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

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