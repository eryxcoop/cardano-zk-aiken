use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Transaction,
}

pub type Redeemer {
  factor1: Int,
  factor2: Int,
}

validator two_prime_factors_number {
  spend(
    datum: Option<Int>,
    redeemer: Redeemer,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    expect Some(two_prime_factors_number) = datum
    assert_factors_are_correct(
      two_prime_factors_number,
      redeemer.factor1,
      redeemer.factor2,
    )?
  }

  else(_) {
    fail
  }
}

fn assert_factors_are_correct(number: Int, factor1: Int, factor2: Int) -> Bool {
  let factor1_is_prime = is_prime(factor1)
  let factor2_is_prime = is_prime(factor2)
  let factors_are_correct = number == factor1 * factor2

  factor1_is_prime && factor2_is_prime && factors_are_correct
}

fn is_prime(n: Int) -> Bool {
  if n > 1 {
    if n != 2 {
      if n % 2 == 1 {
        is_prime_recursive(n, 3)
      } else {
        False
      }
    } else {
      True
    }
  } else {
    False
  }
}

fn is_prime_recursive(n: Int, i: Int) -> Bool {
  if i * i > n {
    True
  } else if n % i == 0 {
    False
  } else {
    is_prime_recursive(n, i + 2)
  }
}

test test_factorization() {
  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  let test_address =
    Address { payment_credential: Script(""), stake_credential: None }

  let test_value = from_lovelace(1000)

  let utxo_output =
    Output {
      address: test_address,
      datum: InlineDatum(35),
      value: test_value,
      reference_script: None,
    }

  let input_utxo = Input { output_reference: utxo_reference, output: utxo_output }

  two_prime_factors_number.spend(
    Some(35),
    Redeemer { factor1: 5, factor2: 7 },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [input_utxo], outputs: [] },
  )
}
