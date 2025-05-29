use ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}
use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Transaction,
}

pub type Datum {
  two_prime_factors_number: Int,
}

pub type Redeemer {
  factor1: Int,
  factor2: Int,
}

validator two_prime_factors_number_succinctness {
  spend(
    datum: Option<Datum>,
    redeemer: Redeemer,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(datum) = datum
    assert_factors_are_correct(
      datum.two_prime_factors_number,
      redeemer.factor1,
      redeemer.factor2,
    )?
  }

  else(_) {
    fail
  }
}

fn assert_factors_are_correct(number: Int, factor1: Int, factor2: Int) -> Bool {
  let pk5: Proof =
    Proof {
      piA: #"8affeb5a4d56ed4542482953f056c7460b8cf8d8aef045dfa0e35b4154a4ffb5cc711392b4fd4d9c60de8d04d83d1de8",
      piB: #"a180ea97f0bc51742ec735ccecbd2e2020164e1f9c6bda53886cf0d186dfc98746e5ae26b04c40584c378d2ed12d515119ba4e7e6088d72c2ecaaadc7b7e9b331b66cf8ff02e1b72d6bcaa75a099ee36debaff0834145c5a18f064a8af7d8745",
      piC: #"a469ce4446a62067c62b398eb09483728aa418d720148e2ea5d25bc57f49c391af538967bf393c5d0b27a97b0dd92b48",
    }

  let pk7: Proof =
    Proof {
      piA: #"84b59f8b3876e9d8b3d39bf8c2f20d206cc955aefde35c6e55b520c07a6706480926e8efb7ab41621fdc5412e2a14aeb",
      piB: #"86618d30b140bf643ad32199a444602d2296e8c3ed82afcd231587a690b98f92f4e38643584a3363084e049c22a68193001d0b73d2d46cc108ae44a00738f2a886f19c7df8a07f6c244617ce0d99c6b2bec70a3f4a666bec64e0d81bc66c4d7b",
      piC: #"84cf820dfba581507bf89fd3a8c13ac028f4d99662dd9398751d3ff318997d437eec7e7194dfa85748fcf011abd12340",
    }

  let factor1_is_prime = is_prime(factor1, pk5)
  //let factor2_is_prime = is_prime(factor2, pk7)
  let factors_are_correct = number == factor1 * factor2

  factor1_is_prime && factors_are_correct
}

fn is_prime(n: Int, proof: Proof) -> Bool {
  let vk: SnarkVerificationKey =
    SnarkVerificationKey {
      nPublic: 1,
      vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
      vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
      vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
      vkDelta: #"a63b9d52a2d9aa48de3ca66dbc829237ad894e770061a862f4418b91634ff1ca886fc0459f0733f0d654022d944123dc0303596b645473be3a95a733503cd857caa22e6ac93457f75cb834bd5a02438081606069def574d3f145a6a1b7a9b1c0",
      vkAlphaBeta: [],
      vkIC: [
        #"96d1c50c8c2dcc5910e43087c81c785175d093399ecb6516e6dade27ac06202d36a7fbb0e446146dad9c7319c084c43a",
        #"ab785c5acee63d1778e61959545c7a3055b702b706923337956630ec89066cbe0c19861fa123efc9d0814e4ba56a4b16",
      ],
    }

  let public_values: List<Int> = [n]

  groth_verify(vk, proof, public_values)
}

test test_factorization() {
  let utxo_datum = Datum { two_prime_factors_number: 35 }

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

  two_prime_factors_number_succinctness.spend(
    Some(utxo_datum),
    Redeemer { factor1: 5, factor2: 7 },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [input_utxo], outputs: [] },
  )
}
