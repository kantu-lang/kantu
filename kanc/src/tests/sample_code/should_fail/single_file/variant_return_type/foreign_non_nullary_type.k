type U {
    .u: U,
}

type T(x: U) {
    .T: T(U.u),
}

type Foo {
    .F: T(U.u),
}
