type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(left: Nat, right: Nat) {
    .Refl(z: Nat): Eq(z, z),
}

let eq_comm = fun eq_comm_(a: Nat, b: Nat, H: Eq(a, b)): Eq(b, a) {
    match H {
        .Refl(_z) => Eq.Refl(a),
    }
};
