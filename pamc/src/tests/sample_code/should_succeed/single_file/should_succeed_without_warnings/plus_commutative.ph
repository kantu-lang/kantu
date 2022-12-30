type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, left: T, right: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a_pred) => Nat.S(plus(a_pred, b)),
    }
};

let plus_S = fun plus_S_(-a: Nat, b: Nat): Eq(Nat, Nat.S(plus(a, b)), plus(a, Nat.S(b))) {
    match a {
        .O => Eq.Refl(Nat, Nat.S(b)),
        .S(a') =>
            match plus_S_(a', b) {
                .Refl(_Nat, _c) => Eq.Refl(Nat, Nat.S(Nat.S(plus(a', b)))),
            },
    }
};

let plus_O = fun plus_O_(-n: Nat): Eq(Nat, plus(n, Nat.O), n) {
    match n {
        .O => Eq.Refl(Nat, Nat.O),
        .S(n') =>
            match plus_O_(n') {
                .Refl(_Nat, _n) => Eq.Refl(Nat, Nat.S(plus(n', Nat.O))),
            },
    }
};

let plus_comm = fun plus_comm_(-a: Nat, b: Nat): Eq(Nat, plus(a, b), plus(b, a)) {
    match a {
        .O =>
            match b {
                .O => Eq.Refl(Nat, Nat.O),
                .S(b') =>
                    match plus_O(b') {
                        .Refl(_Nat, _b) => Eq.Refl(Nat, Nat.S(plus(b', Nat.O))),
                    },
            },
        .S(a') =>
            match b {
                .O =>
                    match plus_O(a') {
                        .Refl(_Nat, _a) => Eq.Refl(Nat, Nat.S(plus(a', Nat.O))),
                    },
                .S(b') =>
                    match plus_S(a', b') {
                        .Refl(_NatSab, _Sab) =>
                            match plus_S(b', a') {
                                .Refl(_NatSba, _Sba) =>
                                    match plus_comm_(a', b') {
                                        .Refl(_NatRec, _Rec) => Eq.Refl(Nat, Nat.S(Nat.S(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};
