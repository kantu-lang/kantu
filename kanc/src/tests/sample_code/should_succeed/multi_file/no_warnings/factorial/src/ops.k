use super.*;

pub let(pack) plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(a') => s(plus(a', b)),
    }
};

pub let(pack) mult = fun mult(-a: Nat, b: Nat): Nat {
    match a {
        o => o,
        s(a') => plus(b, mult(a', b)),
    }
};
