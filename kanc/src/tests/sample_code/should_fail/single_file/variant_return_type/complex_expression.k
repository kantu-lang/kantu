type U {
    .u: U,
}

type Foo {
    .bar(a: U): match U.u { .u => Foo },
}
