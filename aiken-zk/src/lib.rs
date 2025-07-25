use std::fs;
use std::io::ErrorKind;

pub mod lexer;
pub mod parsers;
pub mod token_zk;
pub mod zk_examples;

#[cfg(test)]
mod tests;

pub mod aiken_zk_compiler;
pub mod circom_circuit;
pub mod command_line_interface;
pub mod component_creator;
mod compressed_groth16_proof_bls12_381;
mod compressed_groth16_proof_bls12_381_to_aiken_presenter;
mod meshjs_zk_redeemer_presenter;
mod compressed_groth16_proof_bls12_381_to_meshjs_presenter;

pub fn create_validators_dir_lazy() {
    fs::create_dir("validators")
        .or_else(|error| {
            if error.kind() == ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(error)
            }
        })
        .expect("Couldnt create dir");
}
