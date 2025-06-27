use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

type Redeemer = Void

validator example {
  spend(
    datum: Option<Int>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(b) = datum
    expect _zk_redeemer = offchain assert_eq(priv a, b)
    True
  }

  else(_) {
    fail
  }
}
