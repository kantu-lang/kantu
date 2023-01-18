type Empty {}

type Unit {
    .c: Unit,
}

type Bad {
    .c(f: fun _(_: Unit): Type { forall(b: Bad) { Empty } }(Unit.c)): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .c(f) => f(b),
    }
};

let bad = Bad.c(not_bad);

let empty = not_bad(Bad);
