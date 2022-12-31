type Empty {}

type Not(T: Type) {
    .C(
        T: Type,
        _dummy_so_we_can_test_db_indices: Type,
        f: forall(
            // Thanks to `_dummy_so_we_can_test_db_indices`,
            // `T` should ow have a DB index of 1.
            t: T,
        ) { Empty },
    ): Not(T),
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
