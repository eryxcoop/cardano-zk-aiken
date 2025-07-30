pragma circom 2.1.9;

template Addition() {
    signal input a;
    signal input b;
    signal input c;
    c === a + b;
}