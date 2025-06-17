use aiken/collection/list
use ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}
use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Transaction,
}

pub type Redeemer {
  factor1: Int,
  factor2: Int,
}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

validator two_prime_factors_number_full_succinctness {
  spend(
    datum: Option<Int>,
    zk_redeemer: ZK<Redeemer>,
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
  zk_redeemer: ZK<Redeemer>,
  number: Int
) -> (ZK<Redeemer>, Bool) {
  let redeemer = zk_redeemer.redeemer
  let factor1 = redeemer.factor1
  let factor2 = redeemer.factor2

  let vk: SnarkVerificationKey =
    SnarkVerificationKey {
      nPublic: 1,
      vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
      vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f", 
      vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8", 
      vkDelta: #"b69a82b9e24a8ab0fbf45675ad0ea7af91eb7a5bb8793d00ddb2ea1f6b5165fc0da356b979fe9c2a1b36d5669f95d5210fc63741010629eb0d771ae5299055513f4439a4898ccf2730943ed57491545ed288921522f6f98872c88443b1f0122f",
      vkAlphaBeta: [],
      vkIC: [
        #"a79ab0d2da57b1f49e91ada909216b107d3c6edfb1eaee07eb0e0f70ee8cfb2cc8fcc5cb4880875b038c4b9a3c871602", 
        #"b52b87d8b23b15a6160a7046d315c51fa6737c641c79268a540218c9208054f566d9a2b58a503fddfbc6482d0822927d", 
        #"b1f4aee6c2960becbba178761f364fcdc2fe996af913f1e39519d0f1f19fb5ade722073abeb965e881c8ec75475f08dc",
        #"8202d912866b18ddff3427ff33a1affd93b20a4bd367c18dc18487c44da1eb555817c21f51714bca4505e5f23ec24e75"
        ],
    }

  let public_values: List<Int> = [number, factor1, factor2]
  expect Some(proof) = list.head(zk_redeemer.proofs)
  let is_valid = groth_verify(vk, proof, public_values)

  expect Some(proofs) = list.tail(zk_redeemer.proofs)
  let zk_redeemer = ZK { redeemer: redeemer, proofs }

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
      piA: #"a66840faf8af87a974d1bcf96820f3893870463bfce0140f265bbe2fdf699d7c3e8822c1400d7b1a30390ef5c1e021af", 
      piB: #"897d7b5074b813ff043efa7b5f62a028ead87b8576e8700cbb138ef9d209b4a3365ccd1ffd5eb30f8bbaeac75977a1ca06a9e54ef326e5b60000065406dfb24f64db99e7e38672f21e8a1c67d4ca61db45e9e5887d5db6a3ed7cb40c96256b5e", 
      piC: #"ad6a3410175b686c3982e6cafdcc68414fcda8a769f9f5c06c0b5130146a6c6885ee2a8bffce7e0ce3a5c9cf813ad534"
    }

  let proofs = [factors_are_correct]
  two_prime_factors_number_full_succinctness.spend(
    Some(utxo_datum),
    ZK { proofs, redeemer: Redeemer { factor1: 5, factor2: 7} },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [input_utxo], outputs: [] },
  )
}
