type U {
    .u: U,
}

type Bar(T: Type, x: T) {
    .b: Bar(Bar(U, U.u), Bar.c),
    .c: Bar(U, U.u),
}
