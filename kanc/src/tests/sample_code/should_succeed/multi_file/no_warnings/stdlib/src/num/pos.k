use super.*;

pub type Pos {
    one: Pos,
    extend(left: Pos, right: Bit): Pos,
}

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

pub let(*) succ = fun succ(-a: Pos): Pos {
    match a {
        one => Pos.extend(Pos.one, Bit.zero),
        extend(a_left, a_right) => match a_right {
            zero => Pos.extend(a_left, Bit.one),
            one => Pos.extend(succ(a_left), Bit.zero),
        },
    }
};

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
