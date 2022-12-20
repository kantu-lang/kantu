type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus_(a', b)),
    }
};

let O = Nat.O;

let right = plus(O, O);
let wrong = plus(left: O, right: O);
