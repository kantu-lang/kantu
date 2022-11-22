type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, t: T): Eq(T, t, t),
}

let foo = match Eq.Refl(Nat, Nat.O) {
    .Refl(O) => O,
};
