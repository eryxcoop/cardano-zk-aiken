use crate::cli::subcommand::Subcommand;
use clap::{ArgMatches, Command};
use std::{env, fs, io};
use std::path::{PathBuf, Path};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "milestone_example/"]
struct ProjectTemplate;

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
        // -------------------------------------------------
        // Run [npm i | yarn i | ...] over the desired subdirectories

        // Check for the global installation of aiken, circom, snarkjs. Warn the user if they're not installed
        // Change aiken.toml to match the project name

        fs::create_dir(&project_name).expect("Unable to create working directory");
        Self::copy_embedded_dir(project_name).expect("Failed to populate working directory");
        Self::move_to_working_dir(project_name).expect("Failed to jump into working directory");
        Self::install_javascript_dependencies();

    }

    fn install_javascript_dependencies() {
        let managers = ["npm", "yarn"];
        let usable_manager = managers.iter().find_map(|manager| {
            let result = std::process::Command::new(manager).arg("--version").output();
            match result {
                Ok(_) => Some(manager),
                _ => None
            }
        }).expect("You need to install npm or yarn to start an aiken-zk project");

        let package_manager_status_curve_compress = std::process::Command::new(usable_manager)
            .arg("install")
            .current_dir(Path::new("./curve_compress"))
            .status()
            .expect("Unable to install dependencies in curve_compress");
        if !package_manager_status_curve_compress.success() {
            println!("{usable_manager} installation failed, you need to install manually")
        }

        let package_manager_status_deployment = std::process::Command::new(usable_manager)
            .arg("install")
            .current_dir(Path::new("./deployment"))
            .status()
            .expect("Unable to install dependencies in deployment");
        if !package_manager_status_deployment.success() {
            println!("{usable_manager} installation failed, you need to install manually")
        }

    }

    fn get_arguments(subcommand_matches: &ArgMatches) -> &PathBuf {
        Self::get_argument_value(subcommand_matches, Self::NEW_COMMAND_PROJECT_NAME_ARG_NAME)
    }

    fn copy_embedded_dir(root: &Path) -> Result<(), std::io::Error> {

        let empty_directories = [
            "circuit_inputs",
            "custom_circuits",
            "validators",
            "validators_with_offchain"
        ];

        for dir in empty_directories {
            fs::create_dir_all(root.join(dir))?;
        }

        for file in ProjectTemplate::iter().filter(|f| {
            !empty_directories.contains(&f.split('/').next().unwrap()) &&
            !f.split('/').collect::<Vec<&str>>().contains(&"node_modules") &&
            !f.split('/').collect::<Vec<&str>>().contains(&"examples")
        }) {
            let file_path = Path::new(file.as_ref());
            let out_path = root.join(file_path);

            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let content = ProjectTemplate::get(file.as_ref()).expect("Embedded file missing");
            fs::write(&out_path, &content.data)?;
        }

        Ok(())
    }

    fn move_to_working_dir(working_directory_path: &PathBuf) -> Result<(), io::Error> {
        env::set_current_dir(working_directory_path)
    }
}