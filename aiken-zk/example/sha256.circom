pragma circom 2.1.9;

include "templates/hash.circom";

// template Sha256(nBits) {
// signal input in[nBits];
// signal input out[2];
// ...
// }

component main {public [out]} = Sha256(48);
