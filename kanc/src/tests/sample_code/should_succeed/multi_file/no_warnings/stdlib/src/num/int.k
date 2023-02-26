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

pub let(*) double = fun _(a: Int): Int {
    match a {
        neg(neg_a) => Int.neg(Pos.extend(neg_a, Bit.zero)),
        nat(an) => match an {
            zero => a,
            pos(ap) => Int.nat(Nat.pos(Pos.extend(ap, Bit.zero))),
        },
    }
};

pub let(*) sub_pos = fun sub_pos(a: Pos, -b: Pos): Int {
    match b {
        one => Int.nat(pos.pred(a)),
        extend(b_left, b_right) => match a {
            one => nat.neg(pos.pred(b)),
            extend(a_left, a_right) => match b_right {
                zero => match a_right {
                    zero => double(sub_pos(a_left, b_left)),
                    one => succ(double(sub_pos(a_left, b_left))),
                },
                one => match a_right {
                    one => double(sub_pos(a_left, b_left)),
                    zero => pred(double(sub_pos(a_left, b_left))),
                },
            },
        },
    }
};