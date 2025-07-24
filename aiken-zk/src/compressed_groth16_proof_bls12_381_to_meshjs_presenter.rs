use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;

pub struct CompressedGroth16ProofBls12_381ToMeshJsPresenter {
    proof: CompressedGroth16ProofBls12_381
}

impl CompressedGroth16ProofBls12_381ToMeshJsPresenter {

    pub fn new(compressed_groth16_proof_bls12_381: CompressedGroth16ProofBls12_381) -> Self {
        Self {
            proof: compressed_groth16_proof_bls12_381
        }
    }

    pub fn future_present(&self) -> String {
        format!(
            "        mProof(
\tpiA: #\"{}\",
\tpiB: #\"{}\",
\tpiC: #\"{}\",
}}",
            &self.proof.pi_a_as_byte_string(),
            &self.proof.pi_b_as_byte_string(),
            &self.proof.pi_c_as_byte_string()
        )
    }

    pub fn present(&self) -> String {
        let mut presented_proof = String::new();
        presented_proof += "\t\tmProof(\n";
        presented_proof += &("\t\t\t\"".to_string() + self.proof.pi_a_as_byte_string() + "\",\n");
        presented_proof += &("\t\t\t\"".to_string() + self.proof.pi_b_as_byte_string() + "\",\n");
        presented_proof += &("\t\t\t\"".to_string() + self.proof.pi_c_as_byte_string() + "\",\n");
        presented_proof += &Self::yyy();

        presented_proof
    }

    fn yyy() -> String {
        r#"    ];
}
"#.to_string()
    }
}