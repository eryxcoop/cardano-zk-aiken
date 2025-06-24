pub fn aiken_template_with_body_and_verify_definition(keyword: &str, verify_declaration: &str) -> String {
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

{}"#, keyword, verify_declaration)
}

pub fn verify_declaration(public_input_count: usize) -> String {
    format!(r#"fn zk_verify_or_fail(
        zk_redeemer: ZK<Redeemer>,
        public_inputs: List<Int>
    ) -> ZK<Redeemer> {{
        let redeemer = zk_redeemer.redeemer

        let vk: SnarkVerificationKey =
            SnarkVerificationKey {{
                nPublic: {},
                vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
                vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
                vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
                vkDelta: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
                vkAlphaBeta: [],
                vkIC: [
                #"b42a4610c5c2722df0cae5b696d0e212dd41e471a5246217751ae313dceba2b4d25c1be296ee8e00454027b7c4a45208",
                #"87ef7b539de25c06493f7cd054a78da2819084b7027038d28b31fe88ce0b833f243723fbd9c4e502a3d0c2246aa69560",
                #"a680399022e0bd33fa72396626b4bfc5d1d42e6d9207f3bc64f9fd26a32e5d150ba63a7c28d61db724d362bb9cf96680",
                #"87ac4ff5d2863dd744e3ad397dfde8fe657c09c9c059e25ab8f37b85822eb8f34604d7ca2fe2622d1003ed258319bbf2",
                ],
            }}

        expect Some(proof) = list.head(zk_redeemer.proofs)

        if (!groth_verify(vk, proof, public_inputs)){{
            fail
        }}

        expect Some(proofs) = list.tail(zk_redeemer.proofs)
        let zk_redeemer = ZK {{ redeemer: zk_redeemer.redeemer, proofs }}
    }}"#, public_input_count)
}