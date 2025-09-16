use aiken/builtin.{sha2_256}
use cardano/transaction.{OutputReference, Transaction}

type Redeemer =
  ByteArray

validator example {
  spend(
    _datum: Option<Void>,
    redeemer: Redeemer,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    // sha256("Coffee") = "0xB0E1F06CF1E7D466EE3048EBEF45D85B2D1D31D6164A9B92A6FD0887BA8FFE89"

    // let in = "Coffee"
    let in = redeemer
    let out = sha2_256(in)

    out == #"b0e1f06cf1e7d466ee3048ebef45d85b2d1d31d6164a9b92a6fd0887ba8ffe89"
  }

  else(_) {
    fail
  }
}

test test_example() {
  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  example.spend(
    Some(Void),
    "Coffee",
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}
