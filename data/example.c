int fib( int n ) {
    int a = 1;
    int b = 1;
    while(n>2) {
        int c = b;
        b += a;
        a = c;
        n--;
    }
    return b;
}