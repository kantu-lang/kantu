type Unit {
    .c: Unit,
}

type Foo(T: Type) {
    .bar: Foo(Unit),
}

let unit = match Foo.bar {
    .bar => Unit.c,
};
