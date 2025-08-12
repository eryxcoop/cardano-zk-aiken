use crate::circom_circuit::CircomCircuit;
use crate::presenter::compressed_groth16_proof_bls12_381_to_aiken_presenter::CompressedGroth16ProofBls12_381ToAikenPresenter;
use crate::presenter::meshjs_zk_redeemer_presenter::MeshJsZKRedeemerPresenter;
use crate::cli::subcommand::Subcommand;
use clap::{Arg, ArgMatches, Command};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ProveCommand {}

impl Subcommand for ProveCommand {
    const SUBCOMMAND_NAME: &'static str = "prove";

    fn create_subcommand() -> Command {
        let circom_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_CIRCOM_ARG_NAME);
        let verification_key_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_VK_ARG_NAME);
        let inputs_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_INPUT_ARG_NAME);
        let output_path =
            Self::create_required_argument_with_id(Self::PROVE_COMMAND_OUTPUT_ARG_NAME);

        Command::new(Self::SUBCOMMAND_NAME)
            .subcommand_required(true)
            .subcommand(Self::create_proof_for_aiken_command(
                &circom_path,
                &verification_key_path,
                &inputs_path,
                &output_path,
            ))
            .subcommand(Self::create_library_for_meshjs_command(
                circom_path,
                verification_key_path,
                inputs_path,
                output_path,
            ))
    }

    fn for_name(name: &str) -> bool {
        Self::SUBCOMMAND_NAME.to_string() == name.to_string()
    }

    fn evaluate(&self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(subcommand) => {
                let (match_name, sub_matches) = subcommand;
                let (circom_path, verification_key_path, inputs_path, output_path) =
                    Self::get_prove_arguments(sub_matches);

                if match_name == "aiken" {
                    Self::execute_aiken_prove_command(
                        circom_path,
                        verification_key_path,
                        inputs_path,
                        output_path,
                    );
                } else if match_name == "meshjs" {
                    Self::execute_meshjs_prove_command(
                        circom_path,
                        verification_key_path,
                        inputs_path,
                        output_path,
                    );
                }
            }
            _ => {
                panic!("Unknown or missing subcommand for `prove`");
            }
        }
    }
}

impl ProveCommand {
    const PROVE_COMMAND_CIRCOM_ARG_NAME: &'static str = "circom_path";
    const PROVE_COMMAND_VK_ARG_NAME: &'static str = "verification_key_path";
    const PROVE_COMMAND_INPUT_ARG_NAME: &'static str = "inputs_path";
    const PROVE_COMMAND_OUTPUT_ARG_NAME: &'static str = "output_proof_path";

    fn create_proof_for_aiken_command(
        circom_path: &Arg,
        verification_key_path: &Arg,
        inputs_path: &Arg,
        output_path: &Arg,
    ) -> Command {
        Command::new("aiken")
            .arg(circom_path.clone())
            .arg(verification_key_path.clone())
            .arg(inputs_path.clone())
            .arg(output_path.clone())
    }

    fn create_library_for_meshjs_command(
        circom_path: Arg,
        verification_key_path: Arg,
        inputs_path: Arg,
        output_path: Arg,
    ) -> Command {
        Command::new("meshjs")
            .arg(circom_path.clone())
            .arg(verification_key_path.clone())
            .arg(inputs_path.clone())
            .arg(output_path)
    }

    fn get_prove_arguments(
        subcommand_matches: &ArgMatches,
    ) -> (&PathBuf, &PathBuf, &PathBuf, &PathBuf) {
        let circom_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_CIRCOM_ARG_NAME);
        let verification_key_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_VK_ARG_NAME);
        let inputs_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_INPUT_ARG_NAME);
        let output_path =
            Self::get_argument_value(subcommand_matches, Self::PROVE_COMMAND_OUTPUT_ARG_NAME);
        (circom_path, verification_key_path, inputs_path, output_path)
    }

    fn execute_aiken_prove_command(
        circom_path: &Path,
        verification_key_path: &Path,
        inputs_path: &Path,
        output_path: &Path,
    ) {
        let circom_path_string = circom_path.to_str().unwrap();
        let verification_key_path_string = verification_key_path.to_str().unwrap();
        let inputs_path_string = inputs_path.to_str().unwrap();
        let output_path_string = output_path.to_str().unwrap();

        let circuit = CircomCircuit::from(circom_path_string.to_string());
        let proof =
            circuit.generate_groth16_proof(verification_key_path_string, inputs_path_string);

        let aiken_presenter = CompressedGroth16ProofBls12_381ToAikenPresenter::new(proof);

        let presented_aiken_proof = aiken_presenter.present();
        fs::write(output_path_string, presented_aiken_proof).expect("failed to create output file");
    }

    fn execute_meshjs_prove_command(
        circom_path: &PathBuf,
        verification_key_path: &PathBuf,
        inputs_path: &PathBuf,
        output_path: &PathBuf,
    ) {
        let circom_path_string = circom_path.to_str().unwrap();
        let verification_key_path_string = verification_key_path.to_str().unwrap();
        let inputs_path_string = inputs_path.to_str().unwrap();
        let output_path_string = output_path.to_str().unwrap();

        let circuit = CircomCircuit::from(circom_path_string.to_string());
        let proof =
            circuit.generate_groth16_proof(verification_key_path_string, inputs_path_string);

        let mesh_js_presenter = MeshJsZKRedeemerPresenter::new_for_proof(proof);
        let zk_redeemer = mesh_js_presenter.present();

        fs::write(output_path_string, zk_redeemer).expect("output file write failed");
    }
}
