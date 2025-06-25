pragma circom 2.1.6;

template If() {
    signal input condition;
    signal input assigned;
    signal input true_branch;
    signal input false_branch;

    condition * (1 - condition) === 0;

    signal conditional_true <== condition * true_branch;
    signal conditional_false <== (1-condition) * false_branch;

    assigned === conditional_true + conditional_false;
}