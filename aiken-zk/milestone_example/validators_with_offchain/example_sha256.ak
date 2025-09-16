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
    // string "Coffee" as binary
    // let in = [0,1,0,0,0,0,1,1,0,1,1,0,1,1,1,1,0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,0,1,0,1,1,0,0,1,0,1]
    // sha256("Coffee") = "0xB0E1F06CF1E7D466EE3048EBEF45D85B2D1D31D6164A9B92A6FD0887BA8FFE89"

    // out encoding as 2 integers
    let out = [0xB0E1F06CF1E7D466EE3048EBEF45D85B, 0x2D1D31D6164A9B92A6FD0887BA8FFE89]

    expect _redeemer = offchain sha256(48, priv, pub out)
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