use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct CircomCompiler {
    pub circom_source_code: String,
    pub circom_source_code_path: Option<String>,
}

pub struct Groth16ProofBlsxxx {
    pub piA: String,
    pub piB: String,
    pub piC: String,
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

        compile_circuit(&circom_program_filename_with_extension, output_path);

        let r1cs_path = format!("{}{}.r1cs", output_path, circuit_name);
        let zkey_0 = format!("{}{}_0000.zkey", output_path, circuit_name);
        let zkey_1 = format!("{}{}_0001.zkey", output_path, circuit_name);
        let zkey_2 = format!("{}{}_0002.zkey", output_path, circuit_name);
        let verification_key_zkey = "verification_key.zkey".to_string();
        let verification_key_json = format!("{}verification_key.json", output_path);

        groth16_setup(&r1cs_path, "ceremony.ptau", &zkey_0);
        contribute(&zkey_0, &zkey_1, "1st Contributor Name", rand.0);
        contribute(&zkey_1, &zkey_2, "Second contribution Name", rand.1);
        let hex_entr = "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        beacon(
            &zkey_0,
            &verification_key_zkey,
            hex_entr,
            10,
            "Final Beacon phase2",
        );
        export_verification_key(&verification_key_zkey, &verification_key_json);

        Ok(())
    }

    pub fn generate_aiken_proof(
        circom_path: &str,
        verification_key_path: &str,
        inputs_path: &str,
        output_path: &str,
    ) {
        let build_path = "build/".to_string();
        Self::create_directory_if_not_exists(&build_path);

        compile_circuit(circom_path, &build_path);

        Self::generate_witness(circom_path, inputs_path, &build_path);

        Self::generate_proof(verification_key_path, &build_path);

        Self::convert_proof_to_aiken_proof(output_path, build_path);
    }

    fn convert_proof_to_aiken_proof(output_path: &str, build_path: String) {
        let command_output = Command::new("node")
            .arg("curve_compress/compressedProof.js")
            .arg(build_path + "proof.json")
            .output()
            .expect("failed to finish proof compression");

        let standard_output = String::from_utf8(command_output.stdout).unwrap();
        let compressed_pi_A = &standard_output[..96];
        let compressed_pi_B = &standard_output[96..288];
        let compressed_pi_C = &standard_output[288..384];
        let proof = Groth16ProofBlsxxx {
            piA: compressed_pi_A.to_string(),
            piB: compressed_pi_B.to_string(),
            piC: compressed_pi_C.to_string(),
        };
        let aiken_proof =
            "Proof {\n".to_string() +
            "\tpiA: #\"" +
                &proof.piA +  "\",\n" +
            "\tpiB: #\"" +
                &proof.piB + "\",\n" +
            "\tpiC: #\"" +
                &proof.piC + "\",\n" +
            "}";
        fs::write(output_path, aiken_proof).expect("failed to create output file");
    }

    fn generate_proof(verification_key_path: &str, build_path: &str) {
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
    run_command_or_fail(
        Command::new("circom").args([
            circuit_path,
            "--r1cs",
            "--wasm",
            "--sym",
            "-p",
            "bls12381",
            "-o",
            output_path,
        ]),
        "circom",
    );
}

fn groth16_setup(r1cs_path: &str, ceremony_path: &str, output_zkey: &str) {
    run_command_or_fail(
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
    run_command_or_fail(
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
    run_command_or_fail(
        Command::new("snarkjs").args(["zkey", "export", "verificationkey", zkey, output_json]),
        "export verification key",
    );
}
