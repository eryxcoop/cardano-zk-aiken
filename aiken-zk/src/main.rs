use std::fs;
use std::path::PathBuf;
use clap::{value_parser, Arg, ArgMatches, Command};
use aiken_zk::aiken_zk_compiler::AikenZkCompiler;

fn main() {
    let input = Arg::new("input_path")
        .required(true)
        .value_parser(value_parser!(PathBuf));
    let output = Arg::new("output_path")
        .required(true)
        .value_parser(value_parser!(PathBuf));

    let command = Command::new("aiken-zk")
        .arg(input.clone())
        .arg(output.clone());

    let matches = command.get_matches();
    let input_path = _get_argument_value(&matches, input);
    let output_path = _get_argument_value(&matches, output);

    let input = fs::read_to_string(input_path).unwrap();
    let output = AikenZkCompiler::apply_modifications_to_src_for_token(input, "output".to_string(), ("random1", "random2"));

    fs::write(output_path, output).expect("output file write failed");
}

fn _get_argument_value(subcommand_matches: &ArgMatches, argument: Arg) -> &PathBuf {
    subcommand_matches
        .get_one::<PathBuf>(argument.get_id().to_string().as_str())
        .expect("Value for command not found")
}