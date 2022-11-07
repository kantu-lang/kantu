type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
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

type Eq(T: Type, x: T, y: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

let plus_S = fun plus_S_(-a: Nat, b: Nat): Eq(Nat, Nat.S(plus(a, b)), plus(a, Nat.S(b))) {
    match a {
        .O => Eq.Refl(Nat.S(b)),
        .S(a') =>
            match plus_S_(a', b) {
                .Refl(_c) => Eq.Refl(Nat.S(Nat.S(plus(a', b)))),
            },
    }
};

let plus_O = fun plus_O_(-n: Nat): Eq(plus(n, Nat.O), n) {
    match n {
        .O => Eq.Refl(Nat.O),
        .S(n') =>
            match plus_O_(n') {
                .Refl(_n) => Eq.Refl(Nat.S(plus(n', Nat.O))),
            },
    }
};

let plus_comm = fun plus_comm_(-a: Nat, b: Nat): Eq(plus(a, b), plus(b, a)) {
    match a {
        .O =>
            match b {
                .O => Eq.Refl(Nat.O),
                .S(b') =>
                    match plus_O(b') {
                        .Refl(_b) => Eq.Refl(plus(b', Nat.O)),
                    },
            },
        .S(a') =>
            match b {
                .O =>
                    match plus_O(a') {
                        .Refl(_a) => Eq.Refl(Nat.S(plus(a', Nat.O))),
                    },
                .S(b') =>
                    match plus_S(a', b') {
                        .Refl(_c) =>
                            match plus_S(b', a') {
                                .Refl(_d) =>
                                    match plus_comm_(a', b') {
                                        .Refl(_e) => Eq.Refl(Nat.S(Nat.S(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};

let _2 = Nat.S(Nat.S(Nat.O));
let _3 = Nat.S(_2);

let should_be_5 = plus(_2, _3);
let should_be_9 = square(_3);
let x = List.Cons(Nat, _2, List.Cons(Nat, should_be_5, List.Cons(Nat, should_be_9, List.Nil(Nat))));

let main = square_all(x);
