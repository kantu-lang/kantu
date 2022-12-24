type Empty {}

type Unit {
    .C: Unit,
}

type Bad {
    .C(f: forall(_: Unit) { forall(b: Bad) { Empty } }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .C(f) => f(Unit.C)(b),
    }
};

let bad = Bad.C(not_bad);

let empty = not_bad(Bad);
