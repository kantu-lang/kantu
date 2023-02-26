use super.*;

pub let(*) one = Nat.pos(Pos.one);

pub let(*) eq = fun eq(a: Nat, b: Nat): Bool {
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
pub let(*) neq = fun _(a: Nat, b: Nat): Bool {
    not(eq(a, b))
};

pub let(*) succ = fun succ(a: Nat): Nat {
    match a {
        zero => one,
        pos(ap) => Nat.pos(pos.succ(ap)),
    }
};

pub let(*) pred = fun pred(a: Nat): Int {
    match a {
        zero => Int.neg(Pos.one),
        pos(ap) => Int.nat(pos.pred(ap)),
    }
};

pub let(*) neg = fun _(a: Nat): Int {
    match a {
        zero => Int.nat(Nat.zero),
        pos(ap) => Int.neg(ap),
    }
};

pub let(*) add = fun add(a: Nat, b: Nat): Nat {
    match a {
        zero => b,
        pos(ap) => match b {
            zero => a,
            pos(bp) => Nat.pos(pos.add(ap, bp)),
        },
    }
};

pub let(*) mul = fun mul(a: Nat, b: Nat): Nat {
    match a {
        zero => Nat.zero,
        pos(ap) => match b {
            zero => Nat.zero,
            pos(bp) => Nat.pos(pos.mul(ap, bp)),
        },
    }
};

pub let(*) square = fun _(a: Nat): Nat {
    mul(a, a)
};

/// We'll define 0^0 = 1
pub let(*) pow = fun pow(a: Nat, b: Nat): Nat {
    match b {
        zero => one,
        pos(bp) => match a {
            zero => Nat.zero,
            pos(ap) => Nat.pos(pos.pow(ap, bp)),
        },
    }
};

pub let(*) le = fun le(a: Nat, b: Nat): Bool {
    match a {
        zero => true,
        pos(ap) => match b {
            zero => false,
            pos(bp) => pos.le(ap, bp),
        },
    }
};
pub let(*) lt = fun _(a: Nat, b: Nat): Bool {
    and(le(a, b), neq(a, b))
};

pub let(*) ge = fun _(a: Nat, b: Nat): Bool {
    le(b, a)
};
pub let(*) gt = fun _(a: Nat, b: Nat): Bool {
    lt(b, a)
};
