use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

pub type ZkDatum<datum_type> {
    public_inputs: List<Int>,
    datum: datum_type
}

type Datum = Void
type Redeemer = Void

validator example {
  spend(
    datum: Option<ZkDatum<Datum>>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(datum2) = datum
    let public_inputs = datum2.public_inputs
    expect Some(b) = list.head(public_inputs)
    expect _zk_redeemer = offchain addition(priv a, b, 10)
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
  let pi_value = 7

  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  example.spend(
    Some(ZkDatum {public_inputs: [pi_value], datum: Void}),
    ZK { redeemer: Void, proofs: [proof] },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}