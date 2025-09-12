pragma circom 2.1.9;

template Indexing() {
    signal input l[2];
    signal input val;
    l[0] === val;
}

component main {public [l, val]} = Indexing();