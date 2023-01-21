type Private {
    c: Private,
}

pub type Foo {
    c(_: Private): Foo,
}
