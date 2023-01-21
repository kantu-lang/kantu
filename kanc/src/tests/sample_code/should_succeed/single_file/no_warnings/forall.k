type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let apply = fun apply_(n: Nat, f: forall(m: Nat) { Nat }): Nat {
    f(n)
};

let two = apply(Nat.s(Nat.o), Nat.s);

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(a') => Nat.s(plus_(a', b)),
    }
};

let mult = fun mult_(-a: Nat, b: Nat): Nat {
    match a {
        o => Nat.o,
        s(a') => plus(b, mult_(a', b)),
    }
};

let four = apply(two, fun square_(n: Nat): Nat { mult(n, n) });
