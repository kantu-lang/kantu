type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

type Eq(T: Type, left: T, right: T) {
    refl(T: Type, z: T): Eq(T, z, z),
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(a_pred) => Nat.s(plus(a_pred, b)),
    }
};

let plus_S = fun plus_S_(-a: Nat, b: Nat): Eq(Nat, Nat.s(plus(a, b)), plus(a, Nat.s(b))) {
    match a {
        o => Eq.refl(Nat, Nat.s(b)),
        s(a') =>
            match plus_S_(a', b) {
                refl(_Nat, _c) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b)))),
            },
    }
};

let plus_O = fun plus_O_(-n: Nat): Eq(Nat, plus(n, Nat.o), n) {
    match n {
        o => Eq.refl(Nat, Nat.o),
        s(n') =>
            match plus_O_(n') {
                refl(_Nat, _n) => Eq.refl(Nat, Nat.s(plus(n', Nat.o))),
            },
    }
};

let plus_comm = fun plus_comm_(-a: Nat, b: Nat): Eq(Nat, plus(a, b), plus(b, a)) {
    match a {
        o =>
            match b {
                o => Eq.refl(Nat, Nat.o),
                s(b') =>
                    match plus_O(b') {
                        refl(_Nat, _b) => Eq.refl(Nat, Nat.s(plus(b', Nat.o))),
                    },
            },
        s(a') =>
            match b {
                o =>
                    match plus_O(a') {
                        refl(_Nat, _a) => Eq.refl(Nat, Nat.s(plus(a', Nat.o))),
                    },
                s(b') =>
                    match plus_S(a', b') {
                        refl(_NatSab, _Sab) =>
                            match plus_S(b', a') {
                                refl(_NatSba, _Sba) =>
                                    match plus_comm_(a', b') {
                                        refl(_NatRec, _Rec) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};
