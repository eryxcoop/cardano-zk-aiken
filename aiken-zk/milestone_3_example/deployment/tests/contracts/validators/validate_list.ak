use cardano/transaction.{OutputReference, Transaction}

validator validate_list {
  spend(
    _datum: Option<Void>,
    redeemer: List<Int>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    let list: List<Int> = [1, 2, 3]
    list == redeemer
  }

  else(_) {
    fail
  }
}

test validate_list_test() {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  validate_list.spend(
    None,
    [1, 2, 3],
    placeholder_utxo,
    transaction.placeholder,
  )
}

test validate_list_test_fail() fail {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  validate_list.spend(None, [], placeholder_utxo, transaction.placeholder)
}
