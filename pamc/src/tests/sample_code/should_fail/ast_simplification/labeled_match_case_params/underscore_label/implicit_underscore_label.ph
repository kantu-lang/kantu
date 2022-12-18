type Nat {
    .O: Nat,
    .S(pred~n: Nat): Nat,
}

let pred = fun _(x: Nat): Nat {
    match x {
        .O => Nat.O,
        .S(:_) => x,
    }
};
