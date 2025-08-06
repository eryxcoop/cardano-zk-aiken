use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;

pub struct CompressedGroth16ProofBls12_381ToMeshJsPresenter {
    proof: CompressedGroth16ProofBls12_381
}

impl CompressedGroth16ProofBls12_381ToMeshJsPresenter {

    pub fn new_for(proof: CompressedGroth16ProofBls12_381) -> Self {
        Self {
            proof
        }
    }

    pub fn present(&self) -> String {
        format!(
            "\t\tmProof(
\t\t\t\"{}\",
\t\t\t\"{}\",
\t\t\t\"{}\",
\t\t),
",
            self.proof.pi_a_as_byte_string(),
            self.proof.pi_b_as_byte_string(),
            self.proof.pi_c_as_byte_string()
        )
    }
}