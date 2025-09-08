pragma circom 2.1.9;

/*template EvaluationsXXX(grade, numberOfValuations) {
    signal input coefficients[grade+1];
    signal input rootMerkleTree;
    signal input domain[grade+1];
    signal input evaluations[grade+1];
}*/

// [a0,a1,a2], [x1,x2], [y1,y2]
// P(x) = a0 + a1*x + a2*x^2

// grade = 2
// amountOfEvaluations = 2

template Evaluations(grade, amountOfEvaluations) {
    signal input coefficients[grade+1];
    signal input domain[amountOfEvaluations];
    signal input evaluations[amountOfEvaluations];

    component evaluations[amountOfEvaluations];
    var i;
    for (i=0; i <= amountOfEvaluations; i++) {
        evaluations[i] = Evaluation(grade);
        evaluations[i].coefficients = coefficients;
        evaluations[i].x = domain[i];
        evaluations[i].y = evaluations[i];
    }
}

template Evaluation(grade) {
    signal input coefficients[grade+1];
    signal input x;
    signal input y;

    signal partials[grade+1];
    var i;
    partials[grade] <== coefficients[grade];
    for (i=grade-1; i >= 0; i--) {
        partials[i] <== partials[i+1] * x + coefficients[i];
    }
    y === partials[0];
}

component main { public [coefficients, x, y] } = Evaluate(1);