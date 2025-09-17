pragma circom 2.1.9;

template AssertEq() {
    signal input lhs;
    signal input rhs;
    lhs === rhs;
}