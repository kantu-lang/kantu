type U {
    .U: U,
}

type T(x: U) {
    .T: T(U.U),
}

type Foo {
    .F: T(U.U),
}
