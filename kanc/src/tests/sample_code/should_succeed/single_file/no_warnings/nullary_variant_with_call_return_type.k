type Unit {
    .c: Unit,
}

type Foo(T: Type) {
    .Bar: Foo(Unit),
}

let unit = match Foo.Bar {
    .Bar => Unit.c,
};
