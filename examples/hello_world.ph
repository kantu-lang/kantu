type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a_pred) => Nat.S(plus(a_pred, b)),
    }
};

type Eq(T: Type, left: T, right: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

let _1 = Nat.S(Nat.O);
let _3 = Nat.S(Nat.S(_1));
let _4 = Nat.S(_3);

let foo = fun foo_(a: Nat): Eq(Nat, plus(Nat.S(a), Nat.O), Nat.S(plus(a, Nat.O))) {
    match a {
        .O => Eq.Refl(Nat, Nat.S(plus(a, Nat.O))),
        .S(a') => Eq.Refl(Nat, Nat.S(plus(a, Nat.O))),
    }
};
