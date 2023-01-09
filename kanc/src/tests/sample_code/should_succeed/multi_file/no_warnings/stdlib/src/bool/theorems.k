use super.*;
use pack.Empty;
use pack.Or;
use pack.eq.*;

pub let eq_true_Eq = fun _(a: Bool, b: Bool, H: BoolTrue(ops.eq(a, b))): Eq(Bool, a, b) {
    match a {
        .true => match b {
            .true => Eq.refl(Bool, Bool.true),
            .false => match H { .refl(_, _) => impossible },
        },
        .false => match b {
            .true => match H { .refl(_, _) => impossible },
            .false => Eq.refl(Bool, Bool.false),
        },
    }
};

pub let eq_false_Neq = fun _(a: Bool, b: Bool, H: BoolFalse(ops.eq(a, b))): Neq(Bool, a, b) {
    match a {
        .true => match b {
            .true => match H { .refl(_, _) => impossible },
            .false => fun _(Q: Eq(Bool, a, b)): Empty {
                match Q { .refl(_, _) => impossible }
            },
        },
        .false => match b {
            .true => fun _(Q: Eq(Bool, a, b)): Empty {
                match Q { .refl(_, _) => impossible }
            },
            .false => match H { .refl(_, _) => impossible },
        },
    }
};

pub let eq_refl = fun _(a: Bool): BoolTrue(ops.eq(a, a)) {
    match a {
        .true => bool_true_refl,
        .false => bool_true_refl,
    }
};

pub let Eq_eq_true = fun _(a: Bool, b: Bool, H: Eq(Bool, a, b)): BoolTrue(ops.eq(a, b)) {
    match H {
        .refl(_, _) => check (a = b) { eq_refl(a) },
    }
};

pub let Neq_eq_false = fun _(a: Bool, b: Bool, H: Neq(Bool, a, b)): BoolFalse(ops.eq(a, b)) {
    match ops.eq(a, b) {
        .false => bool_false_refl,
        .true => match H(eq_true_Eq(a, b, bool_true_refl)) {},
    }
};

pub let Eq_dec = fun _(a: Bool, b: Bool): Or(Eq(Bool, a, b), Neq(Bool, a, b)) {
    match ops.eq(a, b) {
        .true => Or.inl(Eq(Bool, a, b), eq_true_Eq(a, b, bool_true_refl), Neq(Bool, a, b)),
        .false => Or.inr(Neq(Bool, a, b), eq_false_Neq(a, b, bool_false_refl), Eq(Bool, a, b)),
    }
};
