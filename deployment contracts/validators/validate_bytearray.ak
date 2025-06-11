use cardano/transaction.{OutputReference, Transaction}

validator validate_list {
  spend(
    _datum: Option<Void>,
    redeemer: ByteArray,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    let bytearray: ByteArray = #"cafe"
    bytearray == redeemer
  }

  else(_) {
    fail
  }
}

test validate_list_test() {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  validate_list.spend(None, #"cafe", placeholder_utxo, transaction.placeholder)
}

test validate_list_test_fail() fail {
  let placeholder_utxo = OutputReference { transaction_id: "", output_index: 0 }
  validate_list.spend(None, #"abcd", placeholder_utxo, transaction.placeholder)
}
