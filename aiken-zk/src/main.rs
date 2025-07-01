use aiken_zk::aiken_zk_compiler::AikenZkCompiler;
use clap::{Arg, ArgMatches, Command, value_parser};
use std::fs;
use std::path::PathBuf;

fn main() {
    let build_subcommand = create_build_subcommand();
    let main_command = Command::new("aiken-zk")
        .subcommand_required(true)
        .subcommand(build_subcommand.clone());

    let main_command_matches = main_command.get_matches();
    if let Some(subcommand_matches) = main_command_matches.subcommand_matches("build") {
        let source_path = _get_argument_value(&subcommand_matches, "source_path");
        let output_path = _get_argument_value(&subcommand_matches, "output_path");

        let source_offchain_aiken = fs::read_to_string(source_path).unwrap();
        let output_zk_aiken = AikenZkCompiler::apply_modifications_to_src_for_token(
            source_offchain_aiken,
            "output".to_string(),
            ("random1", "random2"),
        );

        fs::write(output_path, output_zk_aiken).expect("output file write failed");   
    }
}

fn _get_argument_value<'a>(subcommand_matches: &'a ArgMatches, argument_id: &str) -> &'a PathBuf {
    subcommand_matches
        .get_one::<PathBuf>(argument_id)
        .expect("Value for command not found")
}

fn create_build_subcommand() -> Command {
    let input = Arg::new("source_path")
        .required(true)
        .value_parser(value_parser!(PathBuf));
    let output = Arg::new("output_path")
        .required(true)
        .value_parser(value_parser!(PathBuf));

    Command::new("build").arg(input.clone()).arg(output.clone())
}
