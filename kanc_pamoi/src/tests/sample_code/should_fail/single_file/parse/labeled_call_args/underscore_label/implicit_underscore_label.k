type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let pred = fun _(a: Nat): Nat {
    match a {
        o => Nat.o,
        s(a') => a',
    }
};

let sub = fun sub_(min~m: Nat, sub~-s: Nat): Nat {
    match s {
        o => m,
        s(s') => pred(sub_(m, s')),
    }
};

let test = sub(:_);
