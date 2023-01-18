type U {
    .u: U,
}

type Foo {
    .Bar(a: U): match U.u { .u => Foo },
}
