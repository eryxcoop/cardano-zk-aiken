use aiken_zk::command_line_interface::CommandLineInterface;

fn main() {
    let command_line_interface = CommandLineInterface::create();
    command_line_interface.parse_inputs_and_execute_command();
}
