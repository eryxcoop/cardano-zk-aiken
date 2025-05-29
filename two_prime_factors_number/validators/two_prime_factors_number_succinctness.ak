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
       piA: #"949135c16bda90ad1633a73dc349e3bfa5791d81f66c3bc2de11c58230fff2b24a8c736d72f6e77ad466354c0245a3d9",
       piB: #"8cb4dfcf31b1a950656af753975bc46a76c7ff38f9f024e77e93478c92f9d65a5a6c36ecdb566be81b19a37f6240b9a70f295405af43470931b462ca27189ae51e4aceb38ab457e0c19392dccc1d35cc12f61fec33988f49447c1ddac36e46d2",
       piC: #"99a38d427161b538c189cbb7720aa1b6a84afb7e3824caad44a5e54bb91d4bd1b292ce8e08a7697c89aa2bf2ccc6e739",
     }

  let pk7: Proof =
    Proof {
      piA: #"87b2ffc6ee3e3fad918e629a122cecc7709d758c9d67568ac64e2546d420dedd964911d5ffafb0883381eb12d38e6c99",
      piB: #"84c05d2bc38a0b2229df9471ed92cfa13a2681a1824b4c67c78b3f518447754cd1ec011de7fbcc9f2e0cd3bec8c9b4a212f0b9bb5a29e3097c1de1a62e6aec5d8058b4c2651be334930d59c2ab74a8ffbcce2a625075c103cdf9927bf1e3f2ad",
      piC: #"b6377a604c9e218a004e29c54250e925c84715b762fe3b55e8a98f638f64d9934562f3175f86c021b3caf42ab04e55af",
    }

  let factor1_is_prime = is_prime(factor1, pk5)
  let factor2_is_prime = is_prime(factor2, pk7)
  let factors_are_correct = number == factor1 * factor2

  factor1_is_prime? && factor2_is_prime? && factors_are_correct?
}

fn is_prime(n: Int, proof: Proof) -> Bool {
  let vk: SnarkVerificationKey =
    SnarkVerificationKey {
      nPublic: 1,
      vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
      vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
      vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
      // Delta cambia
      vkDelta: #"a7f309cf308acfa962e775200834fd498d2b7ea89b33a3d8545cdc5ac95f457c26ead2a93fda089efb17b0f77e7103cf03b7157352bf7150fe5009cc63780b2dfcfc92c7b5b0842fce059e895e3bd90a52b69acfad1959a5284b6fb418217e85",
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
