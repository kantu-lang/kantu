use super.*;
use pack.eq.Eq;

pub let eq_Eq = fun _(a: Bool, b: Bool, H: BoolTrue(ops.eq(a, b))): Eq(Bool, a, b) {
    match a {
        .true => match b {
            .true => Eq.refl(Bool, Bool.true),
            .false => match H { .refl => impossible },
        },
        .false => match b {
            .true => match H { .refl => impossible },
            .false => Eq.refl(Bool, Bool.false),
        },
    }
};
