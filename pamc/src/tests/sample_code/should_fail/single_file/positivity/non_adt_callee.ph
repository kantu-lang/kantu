type Empty {}

type Unit {
    .C: Unit,
}

type Bad {
    .C(f: fun _(_: Unit): Type { forall(b: Bad) { Empty } }(Unit.C)): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .C(f) => f(b),
    }
};

let bad = Bad.C(not_bad);

let empty = not_bad(Bad);
