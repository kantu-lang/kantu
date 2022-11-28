type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = fun foo_(_: Nat): Nat {
    _
};
