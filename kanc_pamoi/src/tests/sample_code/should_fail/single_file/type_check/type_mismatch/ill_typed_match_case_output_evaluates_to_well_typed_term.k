type Bool {
    false: Bool,
    true: Bool,
}

type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let identity = fun _(T: Type, t: T): T { t };

let foo = fun _(n: Nat): Nat {
    match n {
        o => identity(Bool, Nat.o),
        s(n') => identity(Bool, Nat.o),
    }
};
