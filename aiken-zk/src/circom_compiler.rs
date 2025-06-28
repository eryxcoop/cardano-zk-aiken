use std::fs;
use std::io::Error;
use std::process::Command;

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
        circom_program_filename: String,
        rand: (&str, &str),
    ) -> Result<(), Error> {
        Command::new("./generate_verification_key.sh")
            .arg(circom_program_filename)
            .arg("ceremony.ptau")
            .arg(rand.0)
            .arg(rand.1)
            .output()?;

        Ok(())
    }
}
