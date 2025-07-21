use std::process::{Command, Output};
pub struct CompressedGroth16ProofBls12_381 {
    piA: String,
    piB: String,
    piC: String,
}

impl CompressedGroth16ProofBls12_381 {
    fn new(piA: &str, piB: &str, piC: &str) -> Self {
        Self {
            piA: piA.to_string(),
            piB: piB.to_string(),
            piC: piC.to_string(),
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

    pub fn piA_as_byte_string(&self) -> &str {
        &self.piA
    }

    pub fn piB_as_byte_string(&self) -> &str {
        &self.piB
    }

    pub fn piC_as_byte_string(&self) -> &str {
        &self.piC
    }
}

