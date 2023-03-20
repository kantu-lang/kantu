type U {
    u: U,
}

type T(x: U) {
    t: T(U.u),
}

type Foo {
    t: T(U.u),
}
