type U {
    u: U,
}

type Bar(T: Type, x: T) {
    b: Bar(Bar(U, U.u), Bar.b),
}
