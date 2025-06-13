pragma circom 2.1.6;

include "./is_prime.circom";

template TwoPrimeFactorsNumber() {
    signal input n;         // 0–97
    signal input factor1;
    signal input factor2;

    component factor1_is_prime = IsPrime();
    factor1_is_prime.n <== factor1;

    component factor2_is_prime = IsPrime();
    factor2_is_prime.n <== factor2;

    n === factor1 * factor2;
}

component main {public [n, factor1, factor2]} = TwoPrimeFactorsNumber();
