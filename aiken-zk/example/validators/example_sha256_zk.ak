use ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}
use cardano/transaction.{OutputReference, Transaction}

type Redeemer =
  Proof

validator example {
  spend(
    _datum: Option<Void>,
    redeemer: Redeemer,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    // sha256("Coffee") = "0xB0E1F06CF1E7D466EE3048EBEF45D85B2D1D31D6164A9B92A6FD0887BA8FFE89"

    // string "Coffee" as binary
    // let in = [0,1,0,0,0,0,1,1,0,1,1,0,1,1,1,1,0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,0,1,0,1,1,0,0,1,0,1]

    // out encoding as 2 integers
    let out =
      [0xb0e1f06cf1e7d466ee3048ebef45d85b, 0x2d1d31d6164a9b92a6fd0887ba8ffe89]

    let proof = redeemer
    prove_sha2_256_for(proof, out)
  }

  else(_) {
    fail
  }
}

fn prove_sha2_256_for(proof, public_inputs) {
  let vk: SnarkVerificationKey =
    SnarkVerificationKey {
      nPublic: 1,
      vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
      vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
      vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
      vkDelta: #"b824005d651bf6d38362f30967f78a33381fde734ebd5687265d2b250f54aa5b10860c1e0cd522e375a7f04178efe69c0747d9ee650176e8376346111c82fd997c3b16604ef241edd84843e9d84a3fd3833bcb2308b5ce4dfa9f465b18dbec7c",
      vkAlphaBeta: [],
      vkIC: [
        #"93c452b7ad2f9bbcd4100fffa506bde3c2b3519c3b1826045ea2bed3450d96994a177712ebd8e4ee631ba3b2f7adc93c",
        #"8762898c57089794897914cd5902cebb7821c30f0b371ba46e98686e777a74b607d4fff9c0d32701e79edcea0a3c83b0",
        #"8819294696cb538ad880029f440da64037ac511fa018293e72c3a2b5b4898031c34283b369b180e57b2d67671b5820f7",
      ],
    }

  groth_verify(vk, proof, public_inputs)
}

test test_example() {
  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  let proof =
    Proof {
      piA: #"b05f0d4533f347d7ff1fa367a42b5e9106d0fdecff354c621dabef90c04553ae6d67ae8c54003b000ad455d1275a159b",
      piB: #"823af7d88b20446fd066e563acba82653c2162d01e16a59ed3525d4432cab80870a9490f280b1e261496e9f4dd61e62702937ad4aba1e0be7270e915a1ce2268ee6fbdbc7b76efe96a48b40681289bcc5fc6d2123e5b459584bf3280edf61784",
      piC: #"8bb9871f13d5d8b891f25be5e6c8295bff809f333c17c62cf4dfda26d6fde9183cbd6d9519ee0e95161e1e9b85f4e07c",
    }

  example.spend(
    Some(Void),
    proof,
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}
