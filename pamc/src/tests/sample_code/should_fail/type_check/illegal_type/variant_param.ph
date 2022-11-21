type U {
    .U: U,
}

type Foo {
    .Bar(x: U.U): Foo,
}
