type U {
    u: U,
}

type Foo(x: U.u) {}
