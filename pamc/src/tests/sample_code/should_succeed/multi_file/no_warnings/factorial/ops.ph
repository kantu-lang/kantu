use super.*;

pub let(pack) plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => S(plus(a', b)),
    }
};

pub let(pack) mult = fun mult(-a: Nat, b: Nat): Nat {
    match a {
        .O => O,
        .S(a') => plus(b, mult(a', b)),
    }
};
