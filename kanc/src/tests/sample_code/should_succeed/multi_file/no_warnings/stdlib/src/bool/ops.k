use super.*;

pub let(*) not = fun _(a: Bool): Bool {
    match a {
        .true => Bool.false,
        .false => Bool.true,
    }
};

pub let(*) and = fun _(a: Bool, b: Bool): Bool {
    match a {
        .true => b,
        .false => Bool.false,
    }
};

pub let(*) or = fun _(a: Bool, b: Bool): Bool {
    match a {
        .true => Bool.true,
        .false => b,
    }
};

pub let(*) eq = fun _(a: Bool, b: Bool): Bool {
    match a {
        .true => b,
        .false => not(b),
    }
};

pub let(*) xor = fun _(a: Bool, b: Bool): Bool {
    not(eq(a, b))
};
