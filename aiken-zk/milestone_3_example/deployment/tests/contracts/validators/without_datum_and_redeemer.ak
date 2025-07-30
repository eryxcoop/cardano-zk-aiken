use cardano/transaction.{OutputReference, Transaction}

validator without_datum_and_redeemer {
  spend(
    _datum: Option<Void>,
    _redeemer: Void,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    True
  }

  else(_) {
    fail
  }
}

test without_datum_and_redeemer_test() {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  without_datum_and_redeemer.spend(
    Some(Void),
    Void,
    placeholder_utxo,
    transaction.placeholder,
  )
}
