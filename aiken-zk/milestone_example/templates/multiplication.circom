pragma circom 2.1.9;

template Multiplication() {
    signal input multiplicand;
    signal input multiplier;
    signal input product;
    product === multiplicand * multiplier;
}