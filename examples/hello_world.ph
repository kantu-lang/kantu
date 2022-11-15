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

let eq_comm = fun eq_comm_(T: Type, a: T, b: T, H: Eq(T, a, b)): Eq(T, b, a) {
    match H {
        .Refl(_T, z) => Eq.Refl(T, z),
    }
};

type IsO(n: Nat) {
    .Triv: IsO(Nat.O),
}

let foo = fun foo_(a: Nat, H: IsO(a)): IsO(a) {
    match a {
        .O => IsO.Triv,
        .S(_a') => H,
    }
};

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

let map = fun map(T: Type, U: Type, -l: List(T), f: forall(v: T) { U }): List(U) {
    match l {
        .Nil(_T) => List.Nil(U),
        .Cons(_T, car, cdr) => List.Cons(U, f(car), map(T, U, cdr, f)),
    }
};

let square_all = fun square_all(l: List(Nat)): List(Nat) { map(Nat, Nat, l, square) };

type Exists(T: Type, P: forall(v: T) { Type }) {
    .witness(T: Type, P: forall(v: T) { Type }, v: T, H: P(v)): Exists(T, P),
}

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
