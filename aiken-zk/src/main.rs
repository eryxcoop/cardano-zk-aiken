use aiken_zk::cli::command_line_interface::CommandLineInterface;

fn main() {
    let command_line_interface = CommandLineInterface::new();
    command_line_interface.parse_inputs_and_execute_command();
}
