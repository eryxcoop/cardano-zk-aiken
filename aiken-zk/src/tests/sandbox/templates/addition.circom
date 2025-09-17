pragma circom 2.1.9;

template Addition() {
    signal input first_addend;
    signal input second_addend;
    signal input sum;
    sum === first_addend + second_addend;
}