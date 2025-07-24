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
        let mut presented_proof = self.file_prefix();
        presented_proof += "\t\tmProof(\n";
        presented_proof += &("\t\t\t\"".to_string() + self.proof.pi_a_as_byte_string() + "\",\n");
        presented_proof += &("\t\t\t\"".to_string() + self.proof.pi_b_as_byte_string() + "\",\n");
        presented_proof += &("\t\t\t\"".to_string() + self.proof.pi_c_as_byte_string() + "\",\n");
        presented_proof += "\t\t),\n";
        presented_proof += &self.file_suffix();

        presented_proof
    }

    fn file_prefix(&self) -> String {
        r#"import {MConStr} from "@meshsdk/common";
import {Data, mConStr0} from "@meshsdk/core";

type Proof = MConStr<any, string[]>;

type ZKRedeemer = MConStr<any, Data[] | Proof[]>;

function mProof(piA: string, piB: string, piC: string): Proof {
    if (piA.length != 96 || piB.length != 192 || piC.length != 96) {
        throw new Error("Wrong proof");
    }

    return mConStr0([piA, piB, piC]);
}

export function mZKRedeemer(redeemer: Data): ZKRedeemer {
    return mConStr0([redeemer, proofs()]);
}

function proofs(): Proof[] {
    return [
"#
            .to_string()
    }
    
    fn file_suffix(&self) -> String {
        r#"    ];
}
"#.to_string()
    }
}