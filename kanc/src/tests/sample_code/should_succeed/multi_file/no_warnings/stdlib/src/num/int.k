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

pub let(*) add = fun _(a: Int, b: Int): Int {
    match a {
        neg(neg_a) => match b {
            neg(neg_b) => Int.neg(pos.add(neg_a, neg_b)),
            nat(bn) => match bn {
                zero => a,
                pos(bp) => sub_pos(bp, neg_a),
            },
        },
        nat(an) => match an {
            zero => b,
            pos(ap) => match b {
                neg(neg_b) => sub_pos(ap, neg_b),
                nat(bn) => Int.nat(nat.add(an, bn)),
            },
        },
    }
};

pub let(*) sub = fun _(a: Int, b: Int): Int {
    add(a, neg(b))
};

pub let(*) mul = fun _(a: Int, b: Int): Int {
    match a {
        neg(neg_a) => match b {
            neg(neg_b) => Int.nat(Nat.pos(pos.mul(neg_a, neg_b))),
            nat(bn) => nat.neg(nat.mul(Nat.pos(neg_a), bn)),
        },
        nat(an) => match b {
            neg(neg_b) => nat.neg(nat.mul(an, Nat.pos(neg_b))),
            nat(bn) => Int.nat(nat.mul(an, bn)),
        },
    }
};

pub let(*) floor_div_bitlist = fun floor_div_bitlist(
    dividend_left: Nat,
    -dividend_right: List(Bit),
    divisor: Pos,
): List(Bit) {
    match dividend_right {
        nil(_) => List.nil(Bit),
        cons(_, car, cdr) => match sub(
            Int.nat(nat.extend_right(dividend_left, car)),
            Int.nat(Nat.pos(divisor)),
        ) {
            // nat.extend_right(dividend_left, car) < divisor
            neg(_) => List.cons(
                Bit,
                Bit.zero,
                floor_div_bitlist(
                    nat.extend_right(dividend_left, car),
                    cdr,
                    divisor,
                ),
            ),

            // nat.extend_right(dividend_left, car) >= divisor
            nat(remainder) => List.cons(
                Bit,
                Bit.one,
                floor_div_bitlist(
                    remainder,
                    cdr,
                    divisor,
                ),
            ),
        },
    }
};

pub let(*) floor_div_pos = fun _(a: Pos, b: Pos): Nat {
    nat.from_bitlist(floor_div_bitlist(Nat.zero, pos.minimal_bitlist(a), b))
};

pub let(*) floor_div_nat = fun _(a: Nat, b: Pos): Nat {
    match a {
        zero => Nat.zero,
        pos(ap) => floor_div_pos(ap, b),
    }
};
