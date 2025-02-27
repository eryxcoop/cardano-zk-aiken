use aiken/collection/dict
use aiken/collection/list
use cardano/transaction.{Transaction, OutputReference, InlineDatum, Output, Input}
use cardano/address.{Address, Script}
use cardano/assets.{from_lovelace, PolicyId, policies}


pub type Datum {
  count: Int
}


validator stt_counter(utxo_ref: OutputReference){

  mint(_redeemer: Data, policy_id: PolicyId, self: Transaction) {
    let Transaction { inputs, outputs, mint, .. } = self
 
    // El piping obtiene todos los tokens asociados a una policy_id y los convierte en pares
    // tokens(self: Value, policy_id: PolicyId) -> Dict<AssetName, Int>
    expect [Pair(_asset_name, quantity)] =
      mint 
      |> assets.tokens(policy_id)
      |> dict.to_pairs()
 
    // Chequea que la transacción que quiere mintear esté gastando el utxo parametrizado
    // Asegurando además que solo
    let is_output_consumed =
      list.any(
        inputs,
        fn(input) { input.output_reference == utxo_ref }
        )

    // List de todas las politicas que tiene ese valor
    expect Some(nft_output) =
      list.find(
        outputs,
        fn(output) { list.has(policies(output.value), policy_id) },
      )

    expect InlineDatum(maybe_new_datum) = nft_output.datum
    expect new_datum: Datum = maybe_new_datum

    is_output_consumed? && (1 == quantity)? && new_datum.count == 7
  }

  // lo que entiendo áca es que en el spend tenemos el old_datum y el actual redeemer.
  //quiero entender si también podría sacar directamente el old_datum de acá y por qué el ejemplo no lo hace
  spend(_old_datum: Option<Datum>, _redeemer: Data, own_ref: OutputReference, self: Transaction) {
    let Transaction { inputs, outputs, .. } = self
 
    expect Some(own_input) =
      list.find(
        inputs,
        fn(input) { input.output_reference == own_ref }
        )

    expect Script(own_script_hash) = own_input.output.address.payment_credential
 
    // Checking if the transaction is signed by the operator.
    // let is_signed_by_operator =
    //   list.has(self.extra_signatories, config.operator)
 
    // One input should hold the STT, with the expected format.
    expect Some(stt_input) =
      list.find(
        inputs,
        fn(input) { list.has(policies(input.output.value), own_script_hash) },
      )
    expect InlineDatum(maybe_old_datum) = stt_input.output.datum
    expect old_datum: Datum = maybe_old_datum
 
    // The STT must be forwarded to an output
    expect Some(stt_output) =
      list.find(
        outputs,
        fn(output) { list.has(policies(output.value), own_script_hash) },
      )
    expect InlineDatum(maybe_new_datum) = stt_output.datum
    expect new_datum: Datum = maybe_new_datum

    expect stt_input.output.address == stt_output.address
 
    // Final validations
    //is_signed_by_operator? && 
    (new_datum.count == old_datum.count + 1)?
  }

}

// test test_stt_counter() {
//   let utxo_datum = Datum { count: 7 }

//   let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

//   let test_address = Address {
//     payment_credential: Script(""),
//     stake_credential: None
//     }

//   let test_value = from_lovelace(1000)

//   let utxo_output = Output { 
//     address: test_address,
//     datum: InlineDatum(utxo_datum),
//     value: test_value,
//     reference_script: None
//     }

//   let new_datum = Datum { count: 8 }

//   let new_utxo = Output { 
//     address: test_address,
//     datum: InlineDatum(new_datum),
//     value: test_value,
//     reference_script: None 
//     }

//   let input_utxo = Input { output_reference: utxo_reference, output: utxo_output }


//   hello_world.spend(
//     Some(utxo_datum),
//     None,
//     utxo_reference,
//     Transaction {
//       ..transaction.placeholder,
//       inputs: [input_utxo],
//       outputs: [new_utxo]
//     }
//   )
// }



