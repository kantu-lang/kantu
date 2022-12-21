// TODO: We'll need to move this file once we add
// misordered call arg warnings.

type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = fun f(~-a: Nat, ~b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => f(:b, a: a'),
    }
};
