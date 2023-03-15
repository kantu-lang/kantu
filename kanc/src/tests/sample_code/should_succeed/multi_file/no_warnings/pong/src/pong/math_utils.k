use super.*;

pub let sign_eq = fun _(a: Sign, b: Sign): Bool {
    match a {
        pos => match b {
            pos => true,
            neg => false,
        },
        neg => match b {
            pos => false,
            neg => true,
        },
    }
};

pub let opt_sign_eq_some = fun _(opt_sign: Opt(Sign), sign: Sign): Bool {
    match opt_sign {
        none(_) => false,
        some(_, sign2) => sign_eq(sign, sign2),
    }
};

pub let sign_int = fun _(sign: Sign, i: Int): Int {
    match sign {
        pos => i,
        neg => int.neg(i),
    }
};

pub let relu = fun _(n: Int): Nat {
    match n {
        neg(_) => Nat.zero,
        nat(nn) => nn,
    }
};

pub let int_to_pos = fun _(n: Int): Pos {
    match n {
        neg(_) => Pos.one,
        nat(nn) => match nn {
            zero => Pos.one,
            pos(np) => np,
        },
    }
};

pub let nat_sub = fun _(min: Nat, sub: Nat): Nat {
    relu(int.sub(
        Int.nat(min),
        Int.nat(sub),
    ))
};

pub let sign_nat = fun _(s: Sign, n: Nat): Int {
    match s {
        pos => Int.nat(n),
        neg => nat.neg(n),
    }
};

pub let nat_mod = fun _(n: Nat, divisor: Pos): Nat {
    relu(int.sub(
        Int.nat(n),
        int.mul(
            int.floor_div(Int.nat(n), divisor),
            Int.nat(Nat.pos(divisor)),
        ),
    ))
};

pub let int_abs = fun _(i: Int): Nat {
    match i {
        neg(neg_i) => Nat.pos(neg_i),
        nat(in) => in,
    }
};

pub let nat_dist = fun _(a: Nat, b: Nat): Nat {
    int_abs(int.sub(Int.nat(a), Int.nat(b)))
};

pub let clamp = fun _(~clampee: Nat, ~min: Nat, ~max: Nat): Nat {
    match nat.le(clampee, min) {
        true => min,
        false => match nat.ge(clampee, max) {
            true => max,
            false => clampee,
        },
    }
};
