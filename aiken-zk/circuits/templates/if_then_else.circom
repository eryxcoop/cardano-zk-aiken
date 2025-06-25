pragma circom 2.1.6;

template IfThenElse() {
    signal input condition;
    signal input assignee;
    signal input then;
    signal input else;

    then === condition * assignee
    else === (1-condition) * assignee;
}

component main {public [condition, assignee, then, else]} = IfThenElse();
