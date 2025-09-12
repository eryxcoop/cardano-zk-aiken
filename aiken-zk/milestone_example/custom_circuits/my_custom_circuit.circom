pragma circom 2.1.9;

include "templates/hash.circom";
include "templates/fibonacci.circom";

template MyCustomCircuit(n) {
    signal input f1;
    signal input f2;
    signal input fn;
    signal input hashedFn;

    component hasher = Poseidon(1);
    hasher.in[0] <== fn;
    hasher.out <== hashedFn;

    component fibonacci = Fibonacci(n);
    fibonacci.a <== f1;
    fibonacci.b <== f2;
    fibonacci.c <== fn;
}

component main {public [f2, hashedFn]} = MyCustomCircuit(5);