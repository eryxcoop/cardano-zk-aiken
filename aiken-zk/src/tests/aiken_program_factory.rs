pub fn aiken_template_with_keyword(keyword: &str) -> String {
    format!(r#"
pub type Redeemer {{
  a: Int,
  b: Int,
  c: Int,
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