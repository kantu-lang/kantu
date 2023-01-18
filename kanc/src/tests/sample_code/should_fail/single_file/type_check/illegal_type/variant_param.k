type U {
    .u: U,
}

type Foo {
    .bar(x: U.u): Foo,
}
