type Unit {
    .C: Unit,
}

type Foo(T: Type) {
    .Bar: Foo(Unit),
}

let unit = match Foo.Bar {
    .Bar => Unit.C,
};
