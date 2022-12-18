type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let pred = fun _(a: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => a',
    }
};

let sub = fun sub_(min~m: Nat, sub~-s: Nat): Nat {
    match s {
        .O => m,
        .S(s') => pred(sub_(m, s')),
    }
};

let test = sub(:_);
