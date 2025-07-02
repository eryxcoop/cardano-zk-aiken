use aiken_zk::command_line_interface::CommandLineInterface;
use aiken_zk::create_validators_dir_lazy;

fn main() {
    create_validators_dir_lazy();
    CommandLineInterface::parse_inputs_and_execute_command();
}


