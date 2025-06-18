pub fn aiken_template_with_keyword(keyword: &str) -> String {
    format!(r#"use aiken/collection/list
use ak_381/groth16.{{Proof, SnarkVerificationKey, groth_verify}}
use cardano/address.{{Address, Script}}
use cardano/assets.{{from_lovelace}}
use cardano/transaction.{{
  InlineDatum, Input, Output, OutputReference, Transaction,
}}

pub type Redeemer {{
  factor1: Int,
  factor2: Int,
}}

pub type ZK<redeemer_type> {{
  redeemer: redeemer_type,
  proofs: List<Proof>,
}}

validator test_validator {{
  spend(
    datum: Option<Int>,
    zk_redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {{
    {}
  }}

  else(_) {{
    fail
  }}
}}
"#, keyword)
}