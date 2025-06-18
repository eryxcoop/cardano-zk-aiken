use std::io::Error;
use std::fs;
use std::process::Command;

pub struct CircomCompiler {
    pub circom_source_code: String,
    pub circom_source_code_path: Option<String>,
}

impl CircomCompiler {
    pub fn from(circom_source_code: String) -> Self {
        Self {
            circom_source_code,
            circom_source_code_path: None
        }
    }

    pub fn save_into_file(&mut self, circom_source_code_path: String) -> Result<(), Error> {
        fs::write(&circom_source_code_path, &self.circom_source_code)?;
        self.circom_source_code_path = Some(circom_source_code_path);
        Ok(())
    }

    pub fn create_verification_key(&mut self, circom_program_filename: String) -> Result<(), Error> {
        Command::new("./compile_proof_verify.sh")
            .arg("-c")
            .arg(circom_program_filename)
            .arg("xx")
            .arg("ceremony.ptau")
            .output()?;

        Ok(())
    }
}