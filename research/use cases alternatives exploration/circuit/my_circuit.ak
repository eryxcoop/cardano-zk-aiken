fn my_circuit(a: PublicInput) -> Int {
    let b = offchain.PrivateInput { value: 2 }
    let c = offchain.Signal {}
    offchain.add(a, b, c)
    let d = offchain.Output {}
    offchain.mul(c, a, d)

    d
}

fn my_circuit() {
    verify()
}

my_circuit.circom
-----------------
template my_circuit() {
    signal input a;
    signal input b;
    signal output d;

    signal c;
    c <== a + b
    d <== c * a
}

component main(public [d]) = my_circuit();

input.json
----------
{a: 5, b: 2}

offchain.PrivateInput
offchain.PublicInput
offchain.Output
sacarse de encima el milestone 2
por qué? porque si avanzamos por el camino
Opciones
1. enfocarnos en seguir el mejor camino para terminar el grant completo antes
2. enfocarnos en sacar el milestone 2 rápido

Si hacemos 1, tal vez hacemos milestone 2-3-4 todos juntos, tardamos 4 meses. Es un problema monetario
tardar 4 meses?

Si hacemos 2, en 1 mes con aire sale, 2 semanas full focus. Cuidar imágen hacia los no técnicos.