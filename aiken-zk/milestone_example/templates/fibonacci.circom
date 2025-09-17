pragma circom 2.1.9;

template Fibonacci(N) {
    assert(N > 2);
    signal input first_fibonacci;
    signal input second_fibonacci;
    signal input nth_fibonacci;

    signal fib[N];
    fib[0] <== first_fibonacci;
    fib[1] <== second_fibonacci;

    for (var i = 2; i < N; i++) {
        fib[i] <== fib[i-1] + fib[i-2];
    }

    fib[N-1] === nth_fibonacci;
}