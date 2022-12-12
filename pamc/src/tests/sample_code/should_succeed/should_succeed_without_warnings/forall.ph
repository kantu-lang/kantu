type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let apply = fun apply_(n: Nat, f: forall(m: Nat) { Nat }): Nat {
    f(n)
};

let two = apply(Nat.S(Nat.O), Nat.S);

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus_(a', b)),
    }
};

let mult = fun mult_(-a: Nat, b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => plus(b, mult_(a', b)),
    }
};

let four = apply(two, fun square_(n: Nat): Nat { mult(n, n) });
