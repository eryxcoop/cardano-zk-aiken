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
    // string "ABCDE"
    let in = [0,1,0,0,0,0,0,1,0,1,0,0,0,0,1,0,0,1,0,0,0,0,1,1,0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,1]
    // sha256 if string "ABCDE"
    // let out_as_bytearray = #"F0393FEBE8BAAA55E32F7BE2A7CC180BF34E52137D99E056C817A9C07B8F239A"
    // byte array of bit encoding out_as_bytearray
    let out_as_integer_list = [1,1,1,1,0,0,0,0,0,0,1,1,1,0,0,1,0,0,1,1,1,1,1,1,1,1,1,0,1,0,1,1,1,1,1,0,1,0,0,0,1,0,1,1,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0,1,0,1,1,1,1,0,0,0,1,1,0,0,1,0,1,1,1,1,0,1,1,1,1,0,1,1,1,1,1,0,0,0,1,0,1,0,1,0,0,1,1,1,1,1,0,0,1,1,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,1,1,1,1,1,1,0,0,1,1,0,1,0,0,1,1,1,0,0,1,0,1,0,0,1,0,0,0,0,1,0,0,1,1,0,1,1,1,1,1,0,1,1,0,0,1,1,0,0,1,1,1,1,0,0,0,0,0,0,1,0,1,0,1,1,0,1,1,0,0,1,0,0,0,0,0,0,1,0,1,1,1,1,0,1,0,1,0,0,1,1,1,0,0,0,0,0,0,0,1,1,1,1,0,1,1,1,0,0,0,1,1,1,1,0,0,1,0,0,0,1,1,1,0,0,1,1,0,1,0]
    expect _redeemer = offchain sha256(40, pub in, pub out_as_integer_list)
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