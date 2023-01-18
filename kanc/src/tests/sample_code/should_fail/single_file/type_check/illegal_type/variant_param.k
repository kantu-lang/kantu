type U {
    .u: U,
}

type Foo {
    .Bar(x: U.u): Foo,
}
