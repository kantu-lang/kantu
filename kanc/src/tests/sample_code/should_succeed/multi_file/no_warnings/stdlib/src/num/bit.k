use super.*;

pub type Bit {
    zero: Bit,
    one: Bit,
}

pub let(*) is_zero = fun _(a: Bit): Bool {
    match a {
        zero => true,
        one => false,
    }
};

pub let(*) is_one = fun _(a: Bit): Bool {
    match a {
        zero => false,
        one => true,
    }
};

pub let(*) eq = fun _(a: Bit, b: Bit): Bool {
    match a {
        zero => is_zero(b),
        one => is_one(b),
    }
};
pub let(*) neq = fun _(a: Bit, b: Bit): Bool {
    not(eq(a, b))
};
