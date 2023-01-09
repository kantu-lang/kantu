use super.*;

pub let(*) plus = fun plus(-a: SimpNat, b: SimpNat): SimpNat {
    match a {
        .o => b,
        .s(a') => SimpNat.s(plus(a', b)),
    }
};

pub let(*) mult = fun mult(-a: SimpNat, b: SimpNat): SimpNat {
    match a {
        .o => SimpNat.o,
        .s(a') => plus(b, mult(a', b)),
    }
};

/// 0^0 = 1
pub let(*) pow = fun pow(a: SimpNat, -b: SimpNat): SimpNat {
    match b {
        .o => SimpNat.s(SimpNat.o),
        .s(b') => mult(a, pow(a, b')),
    }
};

pub let(*) fact = fun fact(-a: SimpNat): SimpNat {
    match a {
        .o => SimpNat.s(SimpNat.o),
        .s(a') => mult(a, fact(a')),
    }
};

pub let(*) saturating_pred = fun _(a: SimpNat): SimpNat {
    match a {
        .o => SimpNat.o,
        .s(a') => a',
    }
};

pub let(*) saturating_sub = fun saturating_sub(a: SimpNat, -b: SimpNat): SimpNat {
    match b {
        .o => a,
        .s(b') => saturating_pred(saturating_sub(a, b')),
    }
};
