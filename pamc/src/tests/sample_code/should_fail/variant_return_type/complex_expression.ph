type U {
    .U: U,
}

type Foo {
    .Bar(a: U): match U.U { .U => Foo },
}
