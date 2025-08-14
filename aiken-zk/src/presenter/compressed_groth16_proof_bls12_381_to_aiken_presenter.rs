use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;

pub struct CompressedGroth16ProofBls12_381ToAikenPresenter {
    proof: CompressedGroth16ProofBls12_381,
}

impl CompressedGroth16ProofBls12_381ToAikenPresenter {
    pub fn new(compressed_groth16_proof_bls12_381: CompressedGroth16ProofBls12_381) -> Self {
        Self {
            proof: compressed_groth16_proof_bls12_381,
        }
    }

    pub fn present(&self) -> String {
        format!(
            "Proof {{
\tpiA: #\"{}\",
\tpiB: #\"{}\",
\tpiC: #\"{}\",
}}",
            &self.proof.pi_a_as_byte_string(),
            &self.proof.pi_b_as_byte_string(),
            &self.proof.pi_c_as_byte_string()
        )
    }
}
