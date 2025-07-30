use cardano/transaction.{OutputReference, Transaction}

validator with_datum_and_redeemer {
  spend(
    datum: Option<Int>,
    redeemer: Int,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(datum_number) = datum

    datum_number == redeemer
  }

  else(_) {
    fail
  }
}

test with_datum_and_redeemer_test() {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  with_datum_and_redeemer.spend(
    Some(42),
    42,
    placeholder_utxo,
    transaction.placeholder,
  )
}

test with_datum_and_redeemer_test_fail() fail {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  with_datum_and_redeemer.spend(
    Some(1),
    2,
    placeholder_utxo,
    transaction.placeholder,
  )
}
