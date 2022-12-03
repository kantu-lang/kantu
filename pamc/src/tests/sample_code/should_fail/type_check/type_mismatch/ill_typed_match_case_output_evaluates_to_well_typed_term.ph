type Bool {
    .False: Bool,
    .True: Bool,
}

type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let identity = fun _(T: Type, t: T): T { t };

let foo = fun _(n: Nat): Nat {
    match n {
        .O => identity(Bool, Nat.O),
        .S(n') => identity(Bool, Nat.O),
    }
};
