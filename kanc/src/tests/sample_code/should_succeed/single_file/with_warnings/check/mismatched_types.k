type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

type Unit {}

let mismatched_types = fun _(m: Nat): Nat {
    match m {
        // WARNING
        o => check (m: Unit) {
            Nat.o
        },
        // WARNING
        s(m') => check (m: ?) {
            Nat.o
        },
    }
};
