use crate::cli::subcommand::Subcommand;
use crate::filename_without_extension_nor_path;
use clap::{ArgMatches, Command};
use colored::Colorize;
use rust_embed::RustEmbed;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(RustEmbed)]
#[folder = "milestone_example/"]
struct ProjectTemplate;

pub struct NewCommand;

impl Subcommand for NewCommand {
    const SUBCOMMAND_NAME: &'static str = "new";

    fn create_subcommand() -> Command {
        let project_name =
            Self::create_required_argument_with_id(Self::NEW_COMMAND_PROJECT_NAME_ARG_NAME);

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
        // -------------------------------------------------

        // Change aiken.toml to match the project name

        fs::create_dir(&project_name).expect("Unable to create working directory");
        Self::copy_embedded_dir(project_name).expect("Failed to populate working directory");
        Self::move_to_working_dir(project_name).expect("Failed to jump into working directory");
        Self::install_javascript_dependencies();
        Self::check_presence_of_dependencies();
    }

    fn install_javascript_dependencies() {
        let usable_manager = Self::obtain_system_usable_package_manager();
        if let Some(usable_manager_name) = usable_manager {
            Self::install_dependencies_in_directory(usable_manager_name, ".");
        } else {
            eprintln!(
                "{}",
                "You don't have npm nor yarn installed or accesible from path. \
            You need to manually install the dependencies / \
            using your typescript package manager"
                    .red()
            )
        }
    }

    fn obtain_system_usable_package_manager() -> Option<&'static str> {
        let managers = ["npm", "yarn"];
        managers.iter().find_map(|&manager| {
            let result = std::process::Command::new(manager)
                .arg("--version")
                .output();
            match result {
                Ok(_) => Some(manager),
                _ => None,
            }
        })
    }

    fn install_dependencies_in_directory(usable_manager: &str, directory: &str) {
        let package_manager_status_curve_compress = std::process::Command::new(usable_manager)
            .arg("install")
            .current_dir(Path::new(directory))
            .status()
            .expect(&format!("Unable to install dependencies in {directory}"));
        if !package_manager_status_curve_compress.success() {
            eprintln!(
                "{usable_manager} installation failed in {directory}, you need to install manually"
            )
        }
    }

    fn get_arguments(subcommand_matches: &ArgMatches) -> &PathBuf {
        Self::get_argument_value(subcommand_matches, Self::NEW_COMMAND_PROJECT_NAME_ARG_NAME)
    }

    fn copy_embedded_dir(root: &Path) -> Result<(), io::Error> {
        let empty_directories = ["circuit_inputs", "validators", "validators_with_offchain"];

        for dir in empty_directories {
            fs::create_dir_all(root.join(dir))?;
        }

        let project_name =
            filename_without_extension_nor_path(String::from(root.to_str().unwrap())).unwrap();

        fs::write(
            root.join(Path::new("aiken.toml")),
            Self::aiken_toml_for_project_with_name(&project_name),
        )
        .expect("Couldn't create aiken.toml");

        for file in ProjectTemplate::iter().filter(|f| {
            let splitted_path = f.split('/').collect::<Vec<&str>>();
            !empty_directories.contains(&f.split('/').next().unwrap())
                && !splitted_path.contains(&"node_modules")
                && !splitted_path.contains(&"examples")
                && !splitted_path.contains(&"CUSTOM_EXAMPLE.md")
                && !splitted_path.contains(&"EXAMPLE.md")
                && !splitted_path.contains(&"aiken.toml")
                && !splitted_path.contains(&"custom_circuits")
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

    fn check_presence_of_dependencies() {
        if std::process::Command::new("aiken")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("{}", "aiken is not installed or accessible from path. Please follow the instructions in https://aiken-lang.org/installation-instructions".red())
        }

        if std::process::Command::new("circom")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("{}", "circom is not installed or accessible from path. Please follow the instructions in https://docs.circom.io/getting-started/installation/".red())
        }

        if std::process::Command::new("snarkjs")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("{}", "snarkjs is not installed or accessible from path. Please install it globally: npm i -g snarkjs".red())
        }
    }

    fn aiken_toml_for_project_with_name(project_name: &str) -> String {
        format!(
            r#"name = "aiken-lang/{project_name}"
version = "0.0.0"
compiler = "v1.1.17"
plutus = "v3"
license = "Apache-2.0"
description = "Aiken contracts for project 'aiken-lang/{project_name}'"

[repository]
user = "aiken-lang"
project = "{project_name}"
platform = "github"

[[dependencies]]
name = "aiken-lang/stdlib"
version = "v2.2.0"
source = "github"

[[dependencies]]
name = "modulo-p/ak-381"
version = "v0.1"
source = "github"

[config]
"#
        )
    }
}
