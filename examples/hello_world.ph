type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Bar(m: Nat) {
    .B(n: Nat): Bar(n),
}

let foo = fun foo_(a: Nat): Bar(a) {
    match a {
        .O => Bar.B(Nat.O),
        .S(a') => Bar.B(a),
    }
};
