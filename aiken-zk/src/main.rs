use aiken_zk::command_line_interface::CommandLineInterface;

fn main() {
    let main_command = CommandLineInterface::create_main_command();
    main_command.parse_inputs_and_execute_command();
}
