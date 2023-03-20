type Empty {}

type Not(T: Type) {
    c(
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
