template Fibonacci(N) {
    assert(N > 2);
    signal input a;
    signal input b;
    signal input c;

    signal fib[N];
    fib[0] <== a;
    fib[1] <== b;

    for (var i = 2; i < N; i++) {
        fib[i] <== fib[i-1] + fib[i-2];
    }

    fib[N-1] === c;
}