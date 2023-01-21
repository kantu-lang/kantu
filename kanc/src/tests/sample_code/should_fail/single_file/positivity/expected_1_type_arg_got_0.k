type Empty {}

type Not(T: Type) {
    c(
        T: Type,
        f: forall(t: T) { Empty },
    ): Not,
}

type Bad {
    c(n: Not(Bad)): Bad,
}

let not_Bad = fun _(b: Bad): Empty {
    match b {
        c(n) =>
            match n {
                c(_Bad, f) => f(b),
            },
    }
};

let Not_Bad = Not.c(Bad, Bad, not_Bad);

let bad = Bad.c(Not_Bad);

let empty = not_Bad(bad);
