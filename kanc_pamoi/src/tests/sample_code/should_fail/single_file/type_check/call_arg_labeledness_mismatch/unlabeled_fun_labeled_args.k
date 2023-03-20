type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(a') => Nat.s(plus_(a', b)),
    }
};

let o = Nat.o;

let right = plus(o, o);
let wrong = plus(left: o, right: o);
