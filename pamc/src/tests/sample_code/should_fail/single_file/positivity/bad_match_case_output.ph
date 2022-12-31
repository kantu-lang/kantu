type Empty {}

type Unit {
    .C: Unit,
}

type Bad {
    .C(f: match Unit.C { .C => forall(b: Bad) { Empty } }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .C(f) => f(b),
    }
};

let bad = Bad.C(not_bad);

let empty = not_bad(Bad);
