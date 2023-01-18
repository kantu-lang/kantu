type U {
    .u: U,
}

type Foo {
    .Bar(a: U): a,
}
