type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let plus = fun plus_(left~-a: Nat, right~b: Nat): Nat {
    match a {
        .o => b,
        .s(a') => Nat.s(plus_(left: a', right: b)),
    }
};

let o = Nat.o;

let right = plus(left: o, right: o);
let wrong = plus(o, o);
