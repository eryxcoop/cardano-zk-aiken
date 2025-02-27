use aiken/collection/list
use cardano/transaction.{OutputReference, Transaction, InlineDatum, Output, Input}
use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace}
 
 pub type Datum {
   count: Int,
 }

 pub type Redeemer {
   count: Int
 }

validator counter {
  spend(
    old_datum: Option<Datum>,
    _redeemer: Option<Redeemer>,
    own_ref: OutputReference,
    self: Transaction,
  ) {


    let Transaction { inputs, outputs, .. } = self
 
    expect Some(Datum { count: old_count }) = old_datum
    
    expect Some(own_input) = 
      list.find(
        inputs,
        fn(input) {input.output_reference == own_ref}   
      )

    expect Some(output) =
      list.find(
        outputs,
        fn(output) {output.address == own_input.output.address}
        )

    expect InlineDatum(maybe_new_datum) = output.datum
    expect new_datum: Datum = maybe_new_datum    

    let new_datum_must_be_next = new_datum.count == old_count + 1
 
    new_datum_must_be_next?
  }
}

test test_counter() {
  let utxo_datum = Datum { count: 7 }

  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  let test_address = Address {
    payment_credential: Script(""),
    stake_credential: None
    }

  let test_value = from_lovelace(1000)

  let utxo_output = Output { 
    address: test_address,
    datum: InlineDatum(utxo_datum),
    value: test_value,
    reference_script: None
    }

  let new_datum = Datum { count: 8 }

  let new_utxo = Output { 
    address: test_address,
    datum: InlineDatum(new_datum),
    value: test_value,
    reference_script: None 
    }

  let input_utxo = Input { output_reference: utxo_reference, output: utxo_output }


  counter.spend(
    Some(utxo_datum),
    None,
    utxo_reference,
    Transaction {
      ..transaction.placeholder,
      inputs: [input_utxo],
      outputs: [new_utxo]
    }
  )
}