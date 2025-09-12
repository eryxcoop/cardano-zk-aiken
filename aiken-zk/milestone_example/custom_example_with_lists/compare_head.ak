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
    let l = [1,2]
    let val = 1
    expect _redeemer = offchain custom("compare_head.circom", [@l, val])
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
  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  example.spend(
    Some(Void),
    ZK { redeemer: Void, proofs: [proof] },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}