type Private {
    .C: Private,
}

pub type Foo {
    .C(_: Private): Foo,
}
