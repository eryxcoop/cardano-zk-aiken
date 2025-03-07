use aiken/collection/dict
use aiken/collection/list
use cardano/address.{Script}
use cardano/assets.{PolicyId, policies}
use cardano/transaction.{InlineDatum, OutputReference, Transaction}
// use config
 
validator counter_stt(utxo_ref: OutputReference) {
  mint(_redeemer: Data, policy_id: PolicyId, self: Transaction) {
    let Transaction { inputs, mint, .. } = self
 
    expect [Pair(_asset_name, quantity)] =
      mint |> assets.tokens(policy_id) |> dict.to_pairs()
 
    let is_output_consumed =
      list.any(inputs, fn(input) { input.output_reference == utxo_ref })
 
    //expect Some(nft_output) =
    //  list.find(
    //    outputs,
    //    fn(output) { list.has(policies(output.value), policy_id) },
    //  )
    //expect InlineDatum(datum) = nft_output.datum
    //expect counter: Int = datum
    //is_output_consumed? && 
    (1 == quantity)? 
    //&& counter == 0
  }

  // "Create the spending part to handle the STT"
    spend(
    _optional_datum: Option<Data>,
    _redeemer: Data,
    own_ref: OutputReference,
    self: Transaction,
  ) {
    let Transaction { inputs, outputs, .. } = self
 
    // Getting the script hash from this validator. Note that since the
    // `mint` handler is defined as part of the same validator, they share
    // the same hash digest. Thus, our `payment_credential` is ALSO our STT
    // minting policy.
    expect Some(own_input) =
      list.find(inputs, fn(input) { input.output_reference == own_ref })
    expect Script(own_script_hash) = own_input.output.address.payment_credential
 
    // // Checking if the transaction is signed by the operator.
    // let is_signed_by_operator =
    //   list.has(self.extra_signatories, config.operator)
 
    // One input should hold the STT, with the expected format.
    expect Some(stt_input) =
      list.find(
        inputs,
        fn(input) { list.has(policies(input.output.value), own_script_hash) },
      )
    expect InlineDatum(input_datum) = stt_input.output.datum
    expect counter_input: Int = input_datum
 
    // The STT must be forwarded to an output
    expect Some(stt_output) =
      list.find(
        outputs,
        fn(output) { list.has(policies(output.value), own_script_hash) },
      )
    expect InlineDatum(output_datum) = stt_output.datum
    expect counter_output: Int = output_datum
    expect stt_input.output.address == stt_output.address
 
    // Final validations
    // is_signed_by_operator? &&
    (counter_output == counter_input + 1)?
  }
}