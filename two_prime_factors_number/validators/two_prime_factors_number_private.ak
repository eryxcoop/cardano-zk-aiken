use aiken/collection/list
use ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}
use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Transaction,
}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

validator two_prime_factors_number_full_succinctness {
  spend(
    datum: Option<Int>,
    zk_redeemer: ZK<Void>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(two_prime_factors_number) = datum
    let (_zk_redeemer, result) =
      assert_factors_are_correct(zk_redeemer, two_prime_factors_number)
    result?
  }

  else(_) {
    fail
  }
}

fn assert_factors_are_correct(
  zk_redeemer: ZK<Void>,
  number: Int
) -> (ZK<Void>, Bool) {
  let vk: SnarkVerificationKey =
    SnarkVerificationKey {
      nPublic: 1,
      vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
      vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f", 
      vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8", 
      vkDelta: #"8ba835c6e6f924b7f2df596b8d54a26d3cac1363e1251d3a5f159dae12fde97d02e7816da49ac60cba15d0086532a832105c3c36b90637066e09e3bdcf564ea89b5d7e8fdd4b318d01dc43be840dd838434a59869fb0f2fbe22f91f6c60de22f",
      vkAlphaBeta: [],
      vkIC: [
        #"a79ab0d2da57b1f49e91ada909216b107d3c6edfb1eaee07eb0e0f70ee8cfb2cc8fcc5cb4880875b038c4b9a3c871602", 
        #"b52b87d8b23b15a6160a7046d315c51fa6737c641c79268a540218c9208054f566d9a2b58a503fddfbc6482d0822927d", 
      ],
    }

  let public_values: List<Int> = [number]
  expect Some(proof) = list.head(zk_redeemer.proofs)
  let is_valid = groth_verify(vk, proof, public_values)

  expect Some(proofs) = list.tail(zk_redeemer.proofs)
  let zk_redeemer = ZK { redeemer: zk_redeemer.redeemer, proofs }

  (zk_redeemer, is_valid)
}

test test_factorization() {

  let utxo_datum = 35

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

  let factors_are_correct: Proof =
    Proof { 
      piA: #"a6ff73ac3b7e373996ad4e45f4392c70de3da99db07933b44f504d230d2cf36b7f6f6e50761cda8d1297b36c5b216ed4", 
      piB: #"91ef5fea8f068dbeb1893af96fd52e2d9b7e25cad9a0639c040e45e0a273a54856c0b37d00fc4601340e7028502da6de07079fe9d07c06122124750b63626e5150716a3bd601ea80420860151e5894c3a8c581636dedda8e0071fddd5aa93b9c", 
      piC: #"a241a69c5c35aa559c9e79e5ee537f56c7fe7d083f7796cce1f8b6d119a1f893739af4a93d7d5743d31201c709cf9b4f"
    }

  let proofs = [factors_are_correct]
  two_prime_factors_number_full_succinctness.spend(
    Some(utxo_datum),
    ZK { proofs, redeemer: Void },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [input_utxo], outputs: [] },
  )
}
