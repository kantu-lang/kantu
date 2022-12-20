type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let plus = fun plus_(left~-a: Nat, right~b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus_(left: a', right: b)),
    }
};

let O = Nat.O;

let right = plus(left: O, right: O);
let wrong = plus(O, O);
