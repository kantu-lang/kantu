use super.*;

pub let(*) eq = fun eq(-a: Int, b: Int): Bool {
    match a {
        neg(ap) => match b {
            neg(bp) => pos.eq(ap, bp),
            nat(_) => false,
        },
        nat(an) => match b {
            neg(_) => false,
            nat(bn) => nat.eq(an, bn),
        },
    }
};
pub let(*) neq = fun _(a: Int, b: Int): Bool {
    not(eq(a, b))
};

pub let(*) succ = fun succ(a: Int): Int {
    match a {
        neg(ap) => nat.neg(pos.pred(ap)),
        nat(an) => Int.nat(nat.succ(an)),
    }
};
