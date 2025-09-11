use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

type Redeemer = Void

validator example {
  spend(
    _datum: Option<Void>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    let in = [1, 2, 3, 4, 5]
    let out = 15084689659490993556398101668230566432614592705711254983366085432266928159258
    // 5 is the length of the in array
    expect _redeemer = offchain poseidon(5, pub in, pub out)
    True
  }

  else(_) {
    fail
  }
}

test test_example() {
  let proof: Proof = Proof {
                     	piA: #"91a6a015c1422894a57a55ec35f9986e7a841c6bbd244d072d7fc29dcd9fb36a190e15d6dd18d55c820e7ec6a4d4ae65",
                     	piB: #"a6e175242b382969e8bf23bf6579eef147240519306b9860a4cea4e089ce4d878d36f46e9b9f28e655c298397130a4fd03d47487fb40c23ca52a1009163d3174f92e6f402883ceb51cffaa57489a3ff346153f2064781440358487e3760920d1",
                     	piC: #"8410cb7fefb2c8c7cbb448839aec5eda4f846a605ca76f83ad126d14943d75fa9b1999e621df4564aebeeef3b9645804",
                     }

  test_proof_is_valid(proof)
}

fn test_proof_is_valid(proof: Proof) -> Bool {
  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  example.spend(
    Some(Void),
    ZK { redeemer: Void, proofs: [proof] },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}