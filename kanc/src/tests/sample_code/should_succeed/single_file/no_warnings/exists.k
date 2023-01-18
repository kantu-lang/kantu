type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, left: T, right: T) {
    .refl(T: Type, z: T): Eq(T, z, z),
}

let identity = fun _(T: Type, t: T): T {
    t
};

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

type Exists(T: Type, P: forall(v: T) { Type }) {
    .witness(T: Type, P: forall(v: T) { Type }, v: T, H: P(v)): Exists(T, P),
}

let three = Nat.s(Nat.s(Nat.s(
    Nat.o,
)));

let nine = Nat.s(Nat.s(Nat.s(
    Nat.s(Nat.s(Nat.s(
        Nat.s(Nat.s(Nat.s(
            Nat.o,
        ))),
    ))),
)));

let nine_is_square = Exists(Nat, fun _(n: Nat): Type { Eq(Nat, square(n), nine) });

let nine_is_square_proof = identity(
    nine_is_square,
    Exists.witness(
        Nat,
        fun _(n: Nat): Type { Eq(Nat, square(n), nine) },
        three,
        Eq.refl(Nat, nine),
    ),
);
