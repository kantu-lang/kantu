use super.*;

pub let one = Nat.pos(Pos.one);

pub let eq = fun eq(a: Nat, b: Nat): Bool {
    match a {
        zero => match b {
            zero => true,
            pos(_) => false,
        },
        pos(ap) => match b {
            zero => false,
            pos(bp) => pos.eq(ap, bp),
        },
    }
};
pub let neq = fun _(a: Nat, b: Nat): Bool {
    not(eq(a, b))
};

pub let succ = fun succ(a: Nat): Nat {
    match a {
        zero => one,
        pos(ap) => Nat.pos(pos.succ(ap)),
    }
};

pub let pred = fun pred(a: Nat): Int {
    match a {
        zero => Int.neg(Pos.one),
        pos(ap) => Int.nat(pos.pred(ap)),
    }
};

pub let from_bit = fun _(a: Bit): Nat {
    match a {
        zero => Nat.zero,
        one => one,
    }
};

pub let extend_right = fun _(a: Nat, right: Bit): Nat {
    match a {
        zero => from_bit(right),
        pos(ap) => Nat.pos(Pos.extend(ap, right)),
    }
};

pub let extend_right_with_bits = fun extend_right_with_bits(a: Nat, -right: List(Bit)): Nat {
    match right {
        nil(_) => a,
        cons(_, car, cdr) => extend_right_with_bits(
            extend_right(a, car),
            cdr,
        ),
    }
};

pub let from_bitlist = fun _(bits: List(Bit)): Nat {
    extend_right_with_bits(Nat.zero, bits)
};

pub let parity = fun _(a: Nat): Bit {
    match a {
        zero => Bit.zero,
        pos(ap) => pos.parity(ap),
    }
};

pub let neg = fun _(a: Nat): Int {
    match a {
        zero => Int.nat(Nat.zero),
        pos(ap) => Int.neg(ap),
    }
};

pub let add = fun add(a: Nat, b: Nat): Nat {
    match a {
        zero => b,
        pos(ap) => match b {
            zero => a,
            pos(bp) => Nat.pos(pos.add(ap, bp)),
        },
    }
};

pub let mul = fun mul(a: Nat, b: Nat): Nat {
    match a {
        zero => Nat.zero,
        pos(ap) => match b {
            zero => Nat.zero,
            pos(bp) => Nat.pos(pos.mul(ap, bp)),
        },
    }
};

pub let square = fun _(a: Nat): Nat {
    mul(a, a)
};

/// We'll define 0^0 = 1
pub let pow = fun _(a: Nat, b: Nat): Nat {
    match b {
        zero => one,
        pos(bp) => match a {
            zero => Nat.zero,
            pos(ap) => Nat.pos(pos.pow(ap, bp)),
        },
    }
};

// Most variables have `mod`-level transparency,
// but since we use `(std.num.nat.)le` to create
// `U8`s, we need this to be globally transparent.
pub let(*) le = fun le(a: Nat, b: Nat): Bool {
    match a {
        zero => true,
        pos(ap) => match b {
            zero => false,
            pos(bp) => pos.le(ap, bp),
        },
    }
};
pub let lt = fun _(a: Nat, b: Nat): Bool {
    and(le(a, b), neq(a, b))
};

pub let ge = fun _(a: Nat, b: Nat): Bool {
    le(b, a)
};
pub let gt = fun _(a: Nat, b: Nat): Bool {
    lt(b, a)
};

pub let cmp = fun _(a: Nat, b: Nat): Ord {
    match lt(a, b) {
        true => Ord.lt,
        false => match eq(a, b) {
            true => Ord.eq,
            false => Ord.gt,
        },
    }
};

pub let min = fun _(a: Nat, b: Nat): Nat {
    match lt(a, b) {
        true => a,
        false => b,
    }
};
pub let max = fun _(a: Nat, b: Nat): Nat {
    match gt(a, b) {
        true => a,
        false => b,
    }
};
