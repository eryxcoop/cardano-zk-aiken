use std::path::Path;

pub mod zk_examples;

#[cfg(test)]
mod tests;

pub mod circom_circuit;
pub mod cli;
pub mod compiler;
pub mod component_creator;
mod compressed_groth16_proof_bls12_381;
mod entropy_generator;
mod presenter;

fn filename_without_extension_nor_path(path: String) -> Option<String> {
    Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from)
}