use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};
use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;

pub struct CircomCompiler {
    pub circom_source_code: String,
    pub circom_source_code_path: Option<String>,
}

impl CircomCompiler {
    pub fn from(circom_source_code: String) -> Self {
        Self {
            circom_source_code,
            circom_source_code_path: None,
        }
    }

    pub fn save_into_file(&mut self, circom_source_code_path: String) -> Result<(), Error> {
        fs::write(&circom_source_code_path, &self.circom_source_code)?;
        self.circom_source_code_path = Some(circom_source_code_path);
        Ok(())
    }

    pub fn create_verification_key(
        &mut self,
        circom_program_filename_with_extension: String,
        rand: (&str, &str),
    ) -> Result<(), Error> {
        let circuit_name = circom_program_filename_with_extension
            .strip_suffix(".circom")
            .unwrap();
        let output_path = "build/";

        fs::create_dir_all(output_path).expect("Failed to create output directory");

        Self::compile_circuit(&circom_program_filename_with_extension, output_path);

        let r1cs_path = format!("{}{}.r1cs", output_path, circuit_name);
        let zkey_0 = format!("{}{}_0000.zkey", output_path, circuit_name);
        let zkey_1 = format!("{}{}_0001.zkey", output_path, circuit_name);
        let zkey_2 = format!("{}{}_0002.zkey", output_path, circuit_name);
        let verification_key_zkey = "verification_key.zkey".to_string();
        let verification_key_json = format!("{}verification_key.json", output_path);

        Self::groth16_setup(&r1cs_path, "ceremony.ptau", &zkey_0);
        Self::contribute(&zkey_0, &zkey_1, "1st Contributor Name", rand.0);
        Self::contribute(&zkey_1, &zkey_2, "Second contribution Name", rand.1);
        let hex_entr = "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        Self::beacon(
            &zkey_0,
            &verification_key_zkey,
            hex_entr,
            10,
            "Final Beacon phase2",
        );
        Self::export_verification_key(&verification_key_zkey, &verification_key_json);

        Ok(())
    }

    pub fn generate_proof(
        circom_path: &str,
        verification_key_path: &str,
        inputs_path: &str,
    ) -> CompressedGroth16ProofBls12_381 {
        let build_path = "build/".to_string();
        Self::create_directory_if_not_exists(&build_path);

        Self::compile_witness_generator(circom_path, &build_path);

        Self::generate_witness(circom_path, inputs_path, &build_path);

        Self::execute_groth16_prove_command(verification_key_path, &build_path);

        CompressedGroth16ProofBls12_381::from_json(&build_path)
    }

    fn execute_groth16_prove_command(verification_key_path: &str, build_path: &str) {
        Command::new("snarkjs")
            .arg("groth16")
            .arg("prove")
            .arg(verification_key_path)
            .arg(build_path.to_string() + "witness.wtns")
            .arg(build_path.to_string() + "proof.json")
            .arg(build_path.to_string() + "public.json")
            .output()
            .unwrap();
    }

    fn generate_witness(circom_path: &str, inputs_path: &str, build_path: &str) {
        let ciruit_name = Path::new(circom_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        Command::new("node")
            .arg(build_path.to_string() + ciruit_name + "_js/generate_witness.js")
            .arg(build_path.to_string() + ciruit_name + "_js/" + ciruit_name + ".wasm")
            .arg(inputs_path)
            .arg(build_path.to_string() + "witness.wtns")
            .output()
            .unwrap();
    }

    fn create_directory_if_not_exists(build_path: &str) {
        fs::create_dir(build_path)
            .or_else(|error| {
                if error.kind() == ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Err(error)
                }
            })
            .expect("Couldnt create directory");
    }

    fn run_command_or_fail(cmd: &mut Command, label: &str) {
        let status = cmd
            .stdout(Stdio::null())
            .status()
            .unwrap_or_else(|_| panic!("Failed to start command '{}'", label));
        if !status.success() {
            panic!(
                "Command '{}' failed with exit code {:?}",
                label,
                status.code()
            );
        }
    }

    fn compile_circuit(circuit_path: &str, output_path: &str) {
        Self::run_command_or_fail(
            Command::new("circom").args([
                circuit_path,
                "--r1cs",
                "-p",
                "bls12381",
                "-o",
                output_path,
            ]),
            "circom",
        );
    }

    fn compile_witness_generator(circuit_path: &str, output_path: &str) {
        Self::run_command_or_fail(
            Command::new("circom").args([
                circuit_path,
                "--wasm",
                "-p",
                "bls12381",
                "-o",
                output_path,
            ]),
            "circom",
        );
    }

    fn groth16_setup(r1cs_path: &str, ceremony_path: &str, output_zkey: &str) {
        Self::run_command_or_fail(
            Command::new("snarkjs").args(["groth16", "setup", r1cs_path, ceremony_path, output_zkey]),
            "groth16 setup",
        );
    }

    fn contribute(input_zkey: &str, output_zkey: &str, name: &str, entropy: &str) {
        let new_entropy = &format!("{}\n", entropy);
        let mut child = Command::new("snarkjs")
            .args([
                "zkey",
                "contribute",
                input_zkey,
                output_zkey,
                &format!("--name={}", name),
                "-v",
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start zkey contribute");

        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(new_entropy.as_bytes())
            .expect("Failed to write entropy");

        let status = child.wait().expect("Failed to wait on zkey contribute");
        if !status.success() {
            panic!(
                "zkey contribute '{}' failed with exit code {:?}",
                name,
                status.code()
            );
        }
    }

    fn beacon(zkey_input: &str, zkey_output: &str, entropy_hex: &str, rounds: u32, name: &str) {
        Self::run_command_or_fail(
            Command::new("snarkjs").args([
                "zkey",
                "beacon",
                zkey_input,
                zkey_output,
                entropy_hex,
                &rounds.to_string(),
                &format!("-n={}", name),
            ]),
            "zkey beacon",
        );
    }

    fn export_verification_key(zkey: &str, output_json: &str) {
        Self::run_command_or_fail(
            Command::new("snarkjs").args(["zkey", "export", "verificationkey", zkey, output_json]),
            "export verification key",
        );
    }
}

