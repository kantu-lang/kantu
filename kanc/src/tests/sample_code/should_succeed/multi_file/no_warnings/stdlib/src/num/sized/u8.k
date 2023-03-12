use super.*;

pub let(*) eq = fun _(a: U8, b: U8): Bool {
    match a {
        u8(an, _) => match b {
            u8(bn, _) => nat.eq(an, bn),
        },
    }
};