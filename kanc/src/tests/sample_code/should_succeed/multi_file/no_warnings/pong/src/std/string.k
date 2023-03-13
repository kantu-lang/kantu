use super.*;
use super.num.*;

pub type String {
    utf8(bytes: List(U8)): String,
}

pub let eq = fun _(s1: String, s2: String): Bool {
    match s1 {
        utf8(s1_bytes) => match s2 {
            utf8(s2_bytes) => list.eq(U8, s1_bytes, s2_bytes, u8.eq),
        },
    }
};
