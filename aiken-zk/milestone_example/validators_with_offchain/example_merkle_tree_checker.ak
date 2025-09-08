use cardano/transaction.{OutputReference, Transaction,}

pub type ZK<redeemer_type> {
  redeemer: redeemer_type,
  proofs: List<Proof>,
}

type Redeemer = Void

validator example {
  spend(
    _datum: Option<Void>,
    redeemer: ZK<Redeemer>,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    // The example tree is:
    // (1, 1) (0, 1)
    //    (10, 7)
    //     (79)
    // The dummy "hash" function for the Merkle tree is H(a,b) = 3a+7b

    // Using the poseidon hash, the merkle tree is
    // (1, 1) (0, 1)
    //   (7af346e2d304279e79e0a9f3023f771294a78acb70e73f90afe27cad401e81,
    //    1bd20834f5de9830c643778a2e88a3a1363c8b9ac083d36d75bf87c49953e65e)
    (17666785367985431650249642360485451519201231170496469694636926545821536978627)

    let leaf = 0
    let merkle_root = 17666785367985431650249642360485451519201231170496469694636926545821536978627
    let path_elements = [
        217234377348884654691879377518794323857294947151490278790710809376325639809,
        12583541437132735734108669866114103169564651237895298778035846191048104863326
    ]
    let path_indices = [0, 1]

    expect _redeemer = offchain merkle_tree_checker(2, leaf, merkle_root , path_elements, path_indices)
    True
  }

  else(_) {
    fail
  }
}

test test_example() {
  let proof: Proof = Proof {
                        piA: "complete with generated piA",
                        piB: "complete with generated piB",
                        piC: "complete with generated piC",
                      }

  test_proof_is_valid(proof)
}

fn test_proof_is_valid(proof: Proof) -> Bool {
  let utxo_reference = OutputReference { transaction_id: "", output_index: 0 }

  example.spend(
    Some(Void),
    ZK { redeemer: Void, proofs: [proof] },
    utxo_reference,
    Transaction { ..transaction.placeholder, inputs: [], outputs: [] },
  )
}