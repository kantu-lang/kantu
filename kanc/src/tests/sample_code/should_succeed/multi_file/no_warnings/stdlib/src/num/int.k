use super.*;

pub let(*) eq = fun eq(-a: Int, b: Int): Bool {
    match a {
        neg(neg_a) => match b {
            neg(neg_b) => pos.eq(neg_a, neg_b),
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

pub let(*) succ = fun _(a: Int): Int {
    match a {
        neg(neg_a) => nat.neg(pos.pred(neg_a)),
        nat(an) => Int.nat(nat.succ(an)),
    }
};

pub let(*) pred = fun _(a: Int): Int {
    match a {
        neg(neg_a) => Int.neg(pos.succ(neg_a)),
        nat(an) => nat.pred(an),
    }
};

pub let(*) neg = fun _(a: Int): Int {
    match a {
        neg(neg_a) => Int.nat(Nat.pos(neg_a)),
        nat(an) => nat.neg(an),
    }
};
