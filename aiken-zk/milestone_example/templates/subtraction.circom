pragma circom 2.1.9;

template Subtraction() {
    signal input minuend;
    signal input subtrahend;
    signal input difference;
    difference === minuend - subtrahend;
}