use crate::aiken_zk_compiler::Groth16CompressedData;

pub fn aiken_template_with_body_and_verify_definition(
    keyword: &str,
    verify_declaration: &str,
) -> String {
    format!(
        r#"pub type ZK<redeemer_type> {{
  redeemer: redeemer_type,
  proofs: List<Proof>,
}}

validator test_validator {{
  spend(
    datum: Option<Int>,
    zk_redeemer: ZK<Void>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {{
    {}
  }}

  else(_) {{
    fail
  }}
}}
{}"#,
        keyword, verify_declaration
    )
}

pub fn verify_declaration(
    public_input_count: usize,
    compressed_vk: Groth16CompressedData,
) -> String {
    let formatted_ic = compressed_vk
        .IC
        .iter()
        .map(|h| format!("                #\"{h}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"
    fn zk_verify_or_fail(
        zk_redeemer: ZK<Redeemer>,
        public_inputs: List<Int>
    ) -> ZK<Redeemer> {{

        let vk: SnarkVerificationKey =
            SnarkVerificationKey {{
                nPublic: {},
                vkAlpha: #"{vkAlpha}",
                vkBeta: #"{vkBeta}",
                vkGamma: #"{vkGamma}",
                vkDelta: #"{vkDelta}",
                vkAlphaBeta: [],
                vkIC: [
{formatted_ic},
                ],
            }}

        expect Some(proof) = list.head(zk_redeemer.proofs)

        if !groth_verify(vk, proof, public_values) {{
          fail
          Void
        }} else {{
          Void
        }}

        expect Some(proofs) = list.tail(zk_redeemer.proofs)
        ZK {{ redeemer: zk_redeemer.redeemer, proofs }}
    }}"#,
        public_input_count,
        vkAlpha = compressed_vk.vk_alpha_1,
        vkBeta = compressed_vk.vk_beta_2,
        vkGamma = compressed_vk.vk_gamma_2,
        vkDelta = compressed_vk.vk_delta_2,
        formatted_ic = formatted_ic,
    )
}
