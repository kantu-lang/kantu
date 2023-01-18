type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .o => b,
        .s(a_pred) => Nat.s(plus(a_pred, b)),
    }
};

let mult = fun mult(-a: Nat, b: Nat): Nat {
    match a {
        .o => Nat.o,
        .s(a_pred) => plus(b, mult(a_pred, b)),
    }
};

let square = fun square(a: Nat): Nat { mult(a, a) };

type Eq(T: Type, left: T, right: T) {
    .refl(T: Type, z: T): Eq(T, z, z),
}

let eq_nat_comm = fun eq_nat_comm_(a: Nat, b: Nat, H: Eq(Nat, a, b)): Eq(Nat, b, a) {
    match H {
        .refl(_Nat, _z) => Eq.refl(Nat, _z),
    }
};

let eq_comm = fun eq_comm_(T: Type, a: T, b: T, H: Eq(T, a, b)): Eq(T, b, a) {
    match H {
        .refl(_T, z) => Eq.refl(T, z),
    }
};

type IsO(n: Nat) {
    .Triv: IsO(Nat.o),
}

let foo = fun foo_(a: Nat, H: IsO(a)): IsO(a) {
    match a {
        .o => IsO.Triv,
        .s(_a') => H,
    }
};

type False {}

type List(T: Type) {
    .nil(T: Type): List(T),
    .cons(T: Type, car: T, cdr: List(T)): List(T),
}

let map = fun map(T: Type, U: Type, -l: List(T), f: forall(v: T) { U }): List(U) {
    match l {
        .nil(_T) => List.nil(U),
        .cons(_T, car, cdr) => List.cons(U, f(car), map(T, U, cdr, f)),
    }
};

let square_all = fun square_all(l: List(Nat)): List(Nat) { map(Nat, Nat, l, square) };

type Exists(T: Type, P: forall(v: T) { Type }) {
    .witness(T: Type, P: forall(v: T) { Type }, v: T, H: P(v)): Exists(T, P),
}

let plus_S = fun plus_S_(-a: Nat, b: Nat): Eq(Nat, Nat.s(plus(a, b)), plus(a, Nat.s(b))) {
    match a {
        .o => Eq.refl(Nat, Nat.s(b)),
        .s(a') =>
            match plus_S_(a', b) {
                .refl(_Nat, _c) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b)))),
            },
    }
};

let plus_O = fun plus_O_(-n: Nat): Eq(Nat, plus(n, Nat.o), n) {
    match n {
        .o => Eq.refl(Nat, Nat.o),
        .s(n') =>
            match plus_O_(n') {
                .refl(_Nat, _n) => Eq.refl(Nat, Nat.s(plus(n', Nat.o))),
            },
    }
};

let plus_comm = fun plus_comm_(-a: Nat, b: Nat): Eq(Nat, plus(a, b), plus(b, a)) {
    match a {
        .o =>
            match b {
                .o => Eq.refl(Nat, Nat.o),
                .s(b') =>
                    match plus_O(b') {
                        .refl(_Nat, _b) => Eq.refl(Nat, Nat.s(plus(b', Nat.o))),
                    },
            },
        .s(a') =>
            match b {
                .o =>
                    match plus_O(a') {
                        .refl(_Nat, _a) => Eq.refl(Nat, Nat.s(plus(a', Nat.o))),
                    },
                .s(b') =>
                    match plus_S(a', b') {
                        .refl(_NatSab, _Sab) =>
                            match plus_S(b', a') {
                                .refl(_NatSba, _Sba) =>
                                    match plus_comm_(a', b') {
                                        .refl(_NatRec, _Rec) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};

let _2 = Nat.s(Nat.s(Nat.o));
let _3 = Nat.s(_2);

let should_be_5 = plus(_2, _3);
let should_be_9 = square(_3);
let x = List.cons(Nat, _2, List.cons(Nat, should_be_5, List.cons(Nat, should_be_9, List.nil(Nat))));

let main = square_all(x);
