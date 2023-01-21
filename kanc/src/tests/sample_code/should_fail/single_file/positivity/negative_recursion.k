type Empty {}

type Bad {
    c(f: forall(b: Bad) { Empty }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        c(f) => f(b),
    }
};

let bad = Bad.c(not_bad);

let empty = not_bad(Bad);
