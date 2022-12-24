type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Unit {}

let mismatched_types = fun _(m: Nat): Nat {
    match m {
        // WARNING
        .O => check (m: Unit) {
            Nat.O
        },
        // WARNING
        .S(m') => check (m: ?) {
            Nat.O
        },
    }
};
