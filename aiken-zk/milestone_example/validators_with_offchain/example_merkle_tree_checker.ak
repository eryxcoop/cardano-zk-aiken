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
    // The example tree is:
    // (1, 1) (0, 1)
    //    (10, 7)
    //     (79)
    // The dummy "hash" function for the Merkle tree is H(a,b) = 3a+7b

    let leaf = 0
    let merkle_root = 79
    let path_elements = [1, 10]
    let path_indices = [0, 1]

    expect _redeemer = offchain merkle_tree_checker(2, leaf, merkle_root , path_elements, path_indices)
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