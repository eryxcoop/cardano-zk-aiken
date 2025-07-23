use std::process::{Command, Output};
pub struct CompressedGroth16ProofBls12_381 {
    pi_a: String,
    pi_b: String,
    pi_c: String,
}

impl CompressedGroth16ProofBls12_381 {
    fn new(pi_a: &str, pi_b: &str, pi_c: &str) -> Self {
        Self {
            pi_a: pi_a.to_string(),
            pi_b: pi_b.to_string(),
            pi_c: pi_c.to_string(),
        }
    }

    pub fn from_json(build_path: &str) -> Self {
        let command_output = Self::execute_curve_compress_command(&build_path);

        let standard_output = String::from_utf8(command_output.stdout).unwrap();

        Self::new(
            &standard_output[..96],
            &standard_output[96..288],
            &standard_output[288..384],
        )
    }

    fn execute_curve_compress_command(build_path: &str) -> Output {
        Command::new("node")
            .arg("curve_compress/compressedProof.js")
            .arg(build_path.to_string() + "proof.json")
            .output()
            .expect("failed to finish proof compression")
    }

    pub fn pi_a_as_byte_string(&self) -> &str {
        &self.pi_a
    }

    pub fn pi_b_as_byte_string(&self) -> &str {
        &self.pi_b
    }

    pub fn pi_c_as_byte_string(&self) -> &str {
        &self.pi_c
    }
}
