type U {
    .U: U,
}

type Foo(x: U.U) {}
