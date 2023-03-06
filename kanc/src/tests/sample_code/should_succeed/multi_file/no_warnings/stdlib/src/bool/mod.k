use super.*;

pub type Bool {
    true: Bool,
    false: Bool,
}
use Bool.*;

pub let(*) not = fun _(a: Bool): Bool {
    match a {
        true => false,
        false => true,
    }
};

pub let(*) and = fun _(a: Bool, b: Bool): Bool {
    match a {
        true => b,
        false => false,
    }
};
pub let(*) nand = fun _(a: Bool, b: Bool): Bool {
    not(and(a, b))
};

pub let(*) or = fun _(a: Bool, b: Bool): Bool {
    match a {
        true => true,
        false => b,
    }
};
pub let(*) nor = fun _(a: Bool, b: Bool): Bool {
    not(or(a, b))
};

pub let(*) xor = fun _(a: Bool, b: Bool): Bool {
    match a {
        true => not(b),
        false => b,
    }
};
pub let(*) xnor = fun _(a: Bool, b: Bool): Bool {
    not(xor(a, b))
};

pub let(*) eq = xnor;
pub let(*) neq = xor;

pub let(*) Trueb = fun _(a: Bool): Type {
    Equal(Bool, true, a)
};
pub let(*) Falseb = fun _(a: Bool): Type {
    Equal(Bool, false, a)
};

pub mod prelude;
