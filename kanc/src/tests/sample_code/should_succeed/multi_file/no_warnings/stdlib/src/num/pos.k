use super.*;

pub let(*) eq = fun eq(-a: Pos, b: Pos): Bool {
    match a {
        one => match b {
            one => true,
            extend(_, _) => false,
        },
        extend(a_left, a_right) => match b {
            one => false,
            extend(b_left, b_right) => and(
                eq(a_left, b_left),
                bit.eq(a_right, b_right),
            ),
        }
    }
};
pub let(*) neq = fun _(a: Pos, b: Pos): Bool {
    not(eq(a, b))
};

pub let(*) minimal_bitlist_plus = fun minimal_bitlist_plus(-a: Pos, right_accumulator: List(Bit)): List(Bit) {
    match a {
        one => List.cons(Bit, Bit.one, right_accumulator),
        extend(a_left, a_right) => minimal_bitlist_plus(
            a_left,
            List.cons(Bit, a_right, right_accumulator),
        ),
    }
};

pub let(*) minimal_bitlist = fun _(a: Pos): List(Bit) {
    minimal_bitlist_plus(a, List.nil(Bit))
};

pub let(*) succ = fun succ(-a: Pos): Pos {
    match a {
        one => Pos.extend(Pos.one, Bit.zero),
        extend(a_left, a_right) => match a_right {
            zero => Pos.extend(a_left, Bit.one),
            one => Pos.extend(succ(a_left), Bit.zero),
        },
    }
};

pub let(*) pred = fun pred(-a: Pos): Nat {
    match a {
        one => Nat.zero,
        extend(a_left, a_right) => Nat.pos(match a_right {
            one => Pos.extend(a_left, Bit.zero),
            zero => match pred(a_left) {
                zero => Pos.one,
                pos(a_left_pred) => Pos.extend(a_left_pred, Bit.one),
            },
        }),
    }
};

pub let(*) neg = Int.neg;

pub let(*) add = fun add(-a: Pos, b: Pos): Pos {
    match a {
        one => succ(b),
        extend(a_left, a_right) => match b {
            one => succ(a),
            extend(b_left, b_right) => match a_right {
                zero => Pos.extend(add(a_left, b_left), b_right),
                one => match b_right {
                    zero => Pos.extend(add(a_left, b_left), Bit.one),
                    one => Pos.extend(
                        succ(add(a_left, b_left)),
                        Bit.zero,
                    ),
                },
            },
        },
    }
};

pub let(*) mul = fun mul(-a: Pos, b: Pos): Pos {
    match a {
        one => b,
        extend(a_left, a_right) => match a_right {
            zero => Pos.extend(mul(a_left, b), Bit.zero),
            one => add(
                Pos.extend(mul(a_left, b), Bit.zero),
                b,
            ),
        },
    }
};

pub let(*) square = fun _(a: Pos): Pos {
    mul(a, a)
};

pub let(*) pow = fun pow(a: Pos, -b: Pos): Pos {
    match b {
        one => a,
        extend(b_left, b_right) => match b_right {
            zero => square(pow(a, b_left)),
            one => mul(
                a,
                square(pow(a, b_left)),
            ),
        },
    }
};

pub let(*) le = fun le(-a: Pos, b: Pos): Bool {
    match a {
        one => true,
        extend(a_left, a_right) => match b {
            one => false,
            extend(b_left, b_right) => match a_right {
                zero => le(a_left, b_left),
                one => match b_right {
                    one => le(a_left, b_left),
                    zero => and(
                        le(a_left, b_left),
                        neq(a_left, b_left),
                    ),
                },
            },
        },
    }
};
pub let(*) lt = fun _(a: Pos, b: Pos): Bool {
    and(le(a, b), neq(a, b))
};

pub let(*) ge = fun _(a: Pos, b: Pos): Bool {
    le(b, a)
};
pub let(*) gt = fun _(a: Pos, b: Pos): Bool {
    lt(b, a)
};

pub let(*) cmp = fun _(a: Pos, b: Pos): Ord {
    match lt(a, b) {
        true => Ord.lt,
        false => match eq(a, b) {
            true => Ord.eq,
            false => Ord.gt,
        },
    }
};
