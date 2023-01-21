// TODO: We'll need to move this file once we add
// misordered call arg warnings.

type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let foo = fun f(~-a: Nat, ~b: Nat): Nat {
    match a {
        o => Nat.o,
        s(a') => f(:b, a: a'),
    }
};
