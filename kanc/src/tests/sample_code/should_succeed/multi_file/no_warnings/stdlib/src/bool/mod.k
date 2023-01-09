use pack.eq.Eq;

pub type Bool {
    .true: Bool,
    .false: Bool,
}

pub let(*) BoolTrue = fun _(b: Bool): Type {
    Eq(Bool, Bool.true, b)
};
pub let(*) bool_true_refl = Eq.refl(Bool, Bool.true);

pub let(*) BoolFalse = fun _(b: Bool): Type {
    Eq(Bool, Bool.false, b)
};
pub let(*) bool_false_refl = Eq.refl(Bool, Bool.false);

pub mod ops;
pub mod theorems;
