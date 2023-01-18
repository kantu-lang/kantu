type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let foo = fun foo_(_: Nat): Nat {
    _
};
