use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

type Redeemer = Int

validator example {
  spend(
    datum: Option<Int>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    let second_fibonacci = redeemer.redeemer
    expect Some(nth_fibonacci) = datum
    expect _redeemer = offchain fibonacci(priv _, second_fibonacci, 5, pub nth_fibonacci)
    True
  }

  else(_) {
    fail
  }
}

test test_example() {
  let proof: Proof = Proof {
                     	piA: "complete with generated piA",
                     	piB: "complete with generated piB",
                     	piC: "complete with generated piC",
                     }

  test_proof_is_valid(proof)
}

fn test_proof_is_valid(proof: Proof) -> Bool {
  let utxo_datum = 13
  let second_fibonacci = 3

  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  example.spend(
    Some(utxo_datum),
    ZK { redeemer: second_fibonacci, proofs: [proof] },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}