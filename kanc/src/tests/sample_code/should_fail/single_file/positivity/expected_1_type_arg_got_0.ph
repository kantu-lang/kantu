type Empty {}

type Not(T: Type) {
    .C(
        T: Type,
        f: forall(t: T) { Empty },
    ): Not,
}

type Bad {
    .C(n: Not(Bad)): Bad,
}

let not_Bad = fun _(b: Bad): Empty {
    match b {
        .C(n) =>
            match n {
                .C(_Bad, f) => f(b),
            },
    }
};

let Not_Bad = Not.C(Bad, Bad, not_Bad);

let bad = Bad.C(Not_Bad);

let empty = not_Bad(bad);
