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

let mult = fun mult(-a: Nat, b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a_pred) => plus(b, mult(a_pred, b)),
    }
};

let square = fun square(a: Nat): Nat { mult(a, a) };

type Eq(T: Type, left: T, right: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

let eq_nat_comm = fun eq_nat_comm_(a: Nat, b: Nat, H: Eq(Nat, a, b)): Eq(Nat, b, a) {
    match H {
        .Refl(_Nat, _z) => Eq.Refl(Nat, _z),
    }
};
