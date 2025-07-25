use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;

pub struct MeshJsZKRedeemerPresenter {
    proof: CompressedGroth16ProofBls12_381
}

impl MeshJsZKRedeemerPresenter {

    pub fn new(compressed_groth16_proof_bls12_381: CompressedGroth16ProofBls12_381) -> Self {
        Self {
            proof: compressed_groth16_proof_bls12_381
        }
    }

    pub fn present(&self) -> String {
        let file_prefix = self.file_prefix();
        let presented_proof = format!(
            "\t\tmProof(
\t\t\t\"{}\",
\t\t\t\"{}\",
\t\t\t\"{}\",
\t\t),
",
            &self.proof.pi_a_as_byte_string(),
            &self.proof.pi_b_as_byte_string(),
            &self.proof.pi_c_as_byte_string()
        );
        let file_sufix = &self.file_suffix();
        format!("{}{}{}", file_prefix, presented_proof, file_sufix)
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