type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type IsO(n: Nat) {
    .Triv: IsO(Nat.O),
}

let foo = fun foo_(a: Nat, H: IsO(a)): IsO(a) {
    match a {
        .O => IsO.Triv,
        .S(_a') => H,
    }
};
