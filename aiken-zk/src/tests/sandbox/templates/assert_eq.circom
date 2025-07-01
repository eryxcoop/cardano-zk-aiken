pragma circom 2.1.9;

template AssertEq() {
    signal input a;
    signal input b;
    a === b;
}