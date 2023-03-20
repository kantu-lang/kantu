type Empty {}

type Wrap(T: Type) {
    c(T: Type): Wrap(T),
}

type Bad {
    c(f: match Wrap.c(Bad) { c(T) => forall(b: T) { Empty } }): Bad,
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        c(f) => f(b),
    }
};

let bad = Bad.c(not_bad);

let empty = not_bad(Bad);
