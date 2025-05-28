use ak_381/groth16.{Proof}
use cardano/transaction.{OutputReference, Transaction}

pub type OffChainInput {
  proof: Proof,
  outputs: Data,
}

pub type OffchainRedeemer<redeemer> {
  offchain: List<OffChainInput>,
  redeemer: redeemer,
}

pub type Datum {
  count: Int,
}

pub type MyRedeemer {
  x: Int,
}

//ZKRedeemer<MyRedeemer>

// redeemer = { count: 3, ZK: [{ proof: sarasa, output: Int }] }
// datum = { hash: "a8sdk9238sdf72k3kj" }

// camino mínimo, test de primalidad

pub type OutputCostoso {
  c1: Int,
  c2: Int,
}

validator counter {
  spend(
    _old_datum: Option<Datum>,
    redeemer: MyRedeemer,
    _own_ref: OutputReference,
    _self: Transaction,
  ) {
    // calculo costoso / succinctness
    // a,b datos derivados del datum, hardcodeos y redeemer
    // podés generar una prueba de groth16
    let a: Int = redeemer.x + 4
    //let b: String = old_datum.hash
    let b: Int = 2

    // calculo_costoso calcula el hash de a
    // y se fija que sea un substring de b
    // ejemplo a = 7
    // hash(7) = "9238sdf72"
    // b = "a8sdk9238sdf72k3kj"
    // c == true
    let c: OutputCostoso = calculo_costoso(a, b)

    //verify(redeemer.proof, verification_key, [a, b, redeemer.ZK[0].output.c1, redeemer.ZK[0].output.c2])
    //let c = {c1: redeemer.ZK[0].output.c1, ;
    c.c1 == 1
  }

  else(_) {
    fail
  }
}

fn calculo_costoso(_a: Int, _b: Int) -> OutputCostoso {
  OutputCostoso { c1: 1, c2: 2 }
}
