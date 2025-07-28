use crate::build_command::BuildCommand;
use crate::prove_command::ProveCommand;
use crate::subcommand::Subcommand;
use clap::Command;

pub struct CommandLineInterface;

macro_rules! execute_subcommand {
    ( $subcommand: ident, $a_command: ident, $( $others_commands:ident ),* ) => {
        {
            let (match_name, matches) = $subcommand;

            if ($a_command::for_name(match_name)) {
                    let command = $a_command {};
                    command.evaluate(matches);
            }
            $(
                else if ($others_commands::for_name(match_name)) {
                    let command = $others_commands {};
                    command.evaluate(matches);
                }
            )*
            else {
                panic!("Unknown command: {}", stringify!(match_name));
            }
        };
    };
}
impl CommandLineInterface {
    pub fn parse_inputs_and_execute_command() {
        let main_command = Self::create_main_command();
        
        let main_command_matches = main_command.get_matches();
        match main_command_matches.subcommand() {
            Some(subcommand) => {
                execute_subcommand!(subcommand, BuildCommand, ProveCommand);
            }
            None => {
                panic!("No command given");
            }
        }
    }

    fn create_main_command() -> Command {
        Command::new("aiken-zk")
            .subcommand_required(true)
            .subcommand(BuildCommand::create_subcommand())
            .subcommand(ProveCommand::create_subcommand())
    }
}
