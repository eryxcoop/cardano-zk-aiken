pragma circom 2.1.9;

// [a0,a1,a2], [x1,x2], [y1,y2]
// P(x) = a0 + a1*x + a2*x^2

// grade = 2
// amountOfEvaluations = 2

template Evaluations(grade, amountOfEvaluations) {
    signal input coefficients[grade+1];
    signal input domain[amountOfEvaluations];
    signal input evaluations[amountOfEvaluations];

    component evaluationCheckers[amountOfEvaluations];
    var i;
    for (i=0; i < amountOfEvaluations; i++) {
        evaluationCheckers[i] = Evaluation(grade);
        evaluationCheckers[i].coefficients <== coefficients;
        evaluationCheckers[i].x <== domain[i];
        evaluationCheckers[i].y === evaluations[i];
    }
}

template Evaluation(grade) {
    signal input coefficients[grade+1];
    signal input x;
    signal output y;

    signal partials[grade+1];
    var i;
    partials[grade] <== coefficients[grade];
    for (i=grade-1; i >= 0; i--) {
        partials[i] <== partials[i+1] * x + coefficients[i];
    }
    y <== partials[0];
}
