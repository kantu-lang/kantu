type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Bar(m: Nat) {
    .B(n: Nat): Bar(n),
}

let bar = fun bar_(a: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => Nat.O,
    }
};

let foo = fun foo_(a: Nat): Bar(a) {
    match a {
        .O => Bar.B(Nat.O),
        .S(a') => Bar.B(a),
    }
};
