type Empty {}

type Unit {
    .c: Unit,
}

type Bad {
    .c(f: forall(_: Unit) { forall(b: Bad) { Empty } }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .c(f) => f(Unit.c)(b),
    }
};

let bad = Bad.c(not_bad);

let empty = not_bad(Bad);
