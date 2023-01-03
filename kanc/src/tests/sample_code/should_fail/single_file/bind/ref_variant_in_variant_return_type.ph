type U {
    .U: U,
}

type Bar(T: Type, x: T) {
    .B: Bar(Bar(U, U.U), Bar.B),
}
