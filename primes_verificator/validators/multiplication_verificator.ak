use ak_381/groth16.{Proof, SnarkVerificationKey, groth_verify}
use cardano/transaction.{OutputReference, Transaction}

validator multiplication_verificator {
  spend(
    _datum: Option<Void>,
    _redeemer: Option<Void>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    let vk: SnarkVerificationKey =
      SnarkVerificationKey {
        nPublic: 1,
        vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
        vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
        vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
        vkDelta: #"98206113a50144574dc701df23935d9f44bf5db8cdfa4d18f4fcffa474f9e31e95ea9a9dd8c1f02b1330ea6884fdca9e0d96f4f4c673ee4f9071eefaa36540302b4b81b0ed952e54a68bad3bff4f5c179772fc0684296d9e6996d5ef90897cce",
        vkAlphaBeta: [],
        vkIC: [
          #"87db49b3c35ae1d3f5b767abf48ca5b73d17c81ad5c50419386a09415e7eba5b7bf50e5d3d2976ec11c31ad4f2ec3477",
          #"a95ec7ba93c9d3c25d74385b6f171ecfe255a6cba464512c852e32a698d98d7f5d6b66577fe29c0bfdc7060161f23ea9",
        ],
      }

    //  Template of Proof
    let pk: Proof =
      Proof {
        piA: #"b8b0eae64a11902f19142b38a90396d32bb827cf20cd03eb85c37ba06d5e0e1045b2a5606bef447c26568d198b9917bd",
        piB: #"97462679997eeea6b629f742762cfc77c764c2d302754f0a4d488412b4c6b3cb983096a937c7271d63010a69ff85789502605e8e1f5bb0649ccfb79efd54d77ff6796e232af58d24d1252ab5f5af78ec637980b683b72a91890ea09931bea682",
        piC: #"85b7c506cf65971efe90e98fa41426de3fb23c078fbee1a72b378cfcda51ab0dfdc619f845c033519a8daaa2d1a03ad4",
      }
    // Template of public values
    let public_values: List<Int> = [42]

    let success = groth_verify(vk, pk, public_values)

    trace @"resultado": success
    success
  }

  else(_) {
    fail
  }
}

test multiplication_verificator_test() {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  multiplication_verificator.spend(
    None,
    None,
    placeholder_utxo,
    transaction.placeholder,
  )
}

test groth_verify_pass_2() {
  //  Template of VK
  let vk: SnarkVerificationKey =
    SnarkVerificationKey {
      nPublic: 1,
      vkAlpha: #"85e3f8a13a670514351a68677ea0e2fc51150daeea496b85a34d97751695e26b2ae4f1a5a3b60e17bb7bfd6d474154c5",
      vkBeta: #"b1abf58f58af5981cd24f996e53626a4157eeed4aa814498885b3a547c35d5efb877834602508255c030708552b353e21631f16475e35b977e39a068ac9fb5bc4c25d383139b721da0a878b663c4df52c94a51f7c06a019bb40324713d2bbf0f",
      vkGamma: #"93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
      vkDelta: #"98206113a50144574dc701df23935d9f44bf5db8cdfa4d18f4fcffa474f9e31e95ea9a9dd8c1f02b1330ea6884fdca9e0d96f4f4c673ee4f9071eefaa36540302b4b81b0ed952e54a68bad3bff4f5c179772fc0684296d9e6996d5ef90897cce",
      vkAlphaBeta: [],
      vkIC: [
        #"87db49b3c35ae1d3f5b767abf48ca5b73d17c81ad5c50419386a09415e7eba5b7bf50e5d3d2976ec11c31ad4f2ec3477",
        #"a95ec7ba93c9d3c25d74385b6f171ecfe255a6cba464512c852e32a698d98d7f5d6b66577fe29c0bfdc7060161f23ea9",
      ],
    }

  //  Template of Proof
  let pk: Proof =
    Proof {
      piA: #"b8b0eae64a11902f19142b38a90396d32bb827cf20cd03eb85c37ba06d5e0e1045b2a5606bef447c26568d198b9917bd",
      piB: #"97462679997eeea6b629f742762cfc77c764c2d302754f0a4d488412b4c6b3cb983096a937c7271d63010a69ff85789502605e8e1f5bb0649ccfb79efd54d77ff6796e232af58d24d1252ab5f5af78ec637980b683b72a91890ea09931bea682",
      piC: #"85b7c506cf65971efe90e98fa41426de3fb23c078fbee1a72b378cfcda51ab0dfdc619f845c033519a8daaa2d1a03ad4",
    }
  // Template of public values
  let public_values: List<Int> = [42]

  groth_verify(vk, pk, public_values)
}
