use aiken/collection/list
use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Redeemer, Transaction,
}

pub type Datum {
  count: Int,
}

datum: {
  a: 5,
  result: 35
}

// redeemer = { count: 3 }
// datum = { hash: "a8sdk9238sdf72k3kj" }

validator counter {
  spend(
    old_datum: Option<Datum>,
    _redeemer: Option<Redeemer>,
    own_ref: OutputReference,
    self: Transaction,
  ) {
    // calculo costoso / succinctness
    // a,b datos derivados del datum, hardcodeos y redeemer
    // podés generar una prueba de groth16
    let a: Int = redeemer.count + 4;
    let b: String = datum.hash

    // calculo_costoso calcula el hash de a
    // y se fija que sea un substring de b
    // ejemplo a = 7
    // hash(7) = "9238sdf72"
    // b = "a8sdk9238sdf72k3kj"
    // c == true
    //let c: Bool = calculo_costoso(a, b);

    verify(redeemer.proof, verification_key, [a, b, redeemer.c])

    c
  }
  else(_) {
    fail
  }
}

test test_counter() {
  let utxo_datum = Datum { count: 11 }

  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  let test_address =
    Address { payment_credential: Script(""), stake_credential: None }

  let test_value = from_lovelace(1000)

  let utxo_output =
    Output {
      address: test_address,
      datum: InlineDatum(utxo_datum),
      value: test_value,
      reference_script: None,
    }

  let new_datum = Datum { count: 12 }

  let new_utxo =
    Output {
      address: test_address,
      datum: InlineDatum(new_datum),
      value: test_value,
      reference_script: None,
    }

  let input_utxo =
    Input { output_reference: utxo_reference, output: utxo_output }

  counter.spend(
    Some(utxo_datum),
    None,
    utxo_reference,
    Transaction {
      ..transaction.placeholder,
      inputs: [input_utxo],
      outputs: [new_utxo],
    },
  )
}
