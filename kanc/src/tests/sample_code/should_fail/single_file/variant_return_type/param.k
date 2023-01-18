type U {
    .u: U,
}

type Foo {
    .bar(a: U): a,
}
