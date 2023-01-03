type U {
    .U: U,
}

type Foo {
    .Bar(a: U): a,
}
