use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
use cardano/transaction.{
    InlineDatum, Input, Output,
}
use aiken/collection/list
use ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}
use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

type Redeemer = Void

validator example {
  spend(
    datum: Option<Int>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(b) = datum
    expect _zk_redeemer = zk_verify_or_fail(redeemer, [b, 10])
    True
  }

  else(_) {
    fail
  }
}

    fn zk_verify_or_fail(
        zk_redeemer: ZK<Redeemer>,
        public_inputs: List<Int>
    ) -> ZK<Redeemer> {

        let vk: SnarkVerificationKey =
            SnarkVerificationKey {
                nPublic: 2,
                vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
                vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
                vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
                vkDelta: #"b824005d651bf6d38362f30967f78a33381fde734ebd5687265d2b250f54aa5b10860c1e0cd522e375a7f04178efe69c0747d9ee650176e8376346111c82fd997c3b16604ef241edd84843e9d84a3fd3833bcb2308b5ce4dfa9f465b18dbec7c",
                vkAlphaBeta: [],
                vkIC: [
                #"af1ca9d68a382928932cd5f1a3dde62489556f42da0c24e6d11191c1b187f147a206da840166e28f1ae73edee0c8b912",
                #"aba434215d34bebf3100b82fb68eaa69328cc6431a26ecc8ef81bffced149a5f7e193587789a1a0c6745b3e963c1989e",
                #"87db49b3c35ae1d3f5b767abf48ca5b73d17c81ad5c50419386a09415e7eba5b7bf50e5d3d2976ec11c31ad4f2ec3477",
                ],
            }

        expect Some(proof) = list.head(zk_redeemer.proofs)

        if !groth_verify(vk, proof, public_inputs) {
          fail
          Void
        } else {
          Void
        }

        expect Some(proofs) = list.tail(zk_redeemer.proofs)
        ZK { redeemer: zk_redeemer.redeemer, proofs }
    }

test test_example() {
  // use generated proof
  let proof: Proof =

    assert_proof_is_valid(proof)
}

fn assert_proof_is_valid(proof: Proof) {

  let utxo_datum = 7

  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  let test_address =
    Address { payment_credential: Script(""), stake_credential: None }

  let test_value = from_lovelace(1000)

  let utxo_output =
    Output {
      address: test_address,
      datum: InlineDatum(utxo_datum),
      value: test_value,
      reference_script: None,
    }

  let input_utxo =
    Input { output_reference: utxo_reference, output: utxo_output }

  let proofs = [proof]
  example.spend(
    Some(utxo_datum),
    ZK { redeemer: Void, proofs: proofs },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [input_utxo], outputs: [] },
  )
}