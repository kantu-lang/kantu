type Empty {}

type Unit {
    c: Unit,
}

type Bad {
    c(f: match Unit.c { .c => forall(b: Bad) { Empty } }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        c(f) => f(b),
    }
};

let bad = Bad.c(not_bad);

let empty = not_bad(Bad);
