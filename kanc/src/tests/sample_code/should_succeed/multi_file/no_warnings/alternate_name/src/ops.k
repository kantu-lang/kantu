pub(pack) use super.Blat as Gnat;

pub let plus = fun plus(-a: Gnat, b: Gnat): Gnat {
    match a {
        .o => b,
        .s(a') => Gnat.s(plus(a', b)),
    }
};
