type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, left: T, right: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

let eq_nat_comm = fun eq_nat_comm_(a: Nat, b: Nat, H: Eq(Nat, a, b)): Eq(Nat, b, a) {
    match H {
        .Refl(_Nat, _z) => Eq.Refl(Nat, _z),
    }
};

let eq_comm = fun eq_comm_(T: Type, a: T, b: T, H: Eq(T, a, b)): Eq(T, b, a) {
    match H {
        .Refl(_T, z) => Eq.Refl(T, z),
    }
};
