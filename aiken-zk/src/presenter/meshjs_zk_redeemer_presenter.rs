use crate::compressed_groth16_proof_bls12_381::CompressedGroth16ProofBls12_381;
use crate::compressed_groth16_proof_bls12_381_to_meshjs_presenter::CompressedGroth16ProofBls12_381ToMeshJsPresenter;

pub struct MeshJsZKRedeemerPresenter {
    proof_presenter: CompressedGroth16ProofBls12_381ToMeshJsPresenter
}

impl MeshJsZKRedeemerPresenter {

    pub fn new_for_proof(compressed_groth16_proof_bls12_381: CompressedGroth16ProofBls12_381) -> Self {
        Self {
            proof_presenter: CompressedGroth16ProofBls12_381ToMeshJsPresenter::new_for(compressed_groth16_proof_bls12_381)
        }
    }

    pub fn present(&self) -> String {
        let file_prefix = self.file_prefix();
        let presented_proof = self.proof_presenter.present();
        let file_suffix = &self.file_suffix();
        format!("{}{}{}", file_prefix, presented_proof, file_suffix)
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