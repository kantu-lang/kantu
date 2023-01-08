type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, c: T): Eq(T, c, c),
}

let foo = fun bar_(x: Nat): Nat {
    match
        match x {
            .S(problem) => Eq.Refl(Nat, Nat.S(problem)),
            .O => Eq.Refl(Nat, Nat.O),
        }
    {
        .Refl(_U, _x) => Nat.O,
    }
};
