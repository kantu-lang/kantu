type Empty {}

type Wrap(T: Type) {
    .C(T: Type): Wrap(T),
}

type Bad {
    .C(f: match Wrap.C(Bad) { .C(T) => forall(b: T) { Empty } }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .C(f) => f(b),
    }
};

let bad = Bad.C(not_bad);

let empty = not_bad(Bad);
