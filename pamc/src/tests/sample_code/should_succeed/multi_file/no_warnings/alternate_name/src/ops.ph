pub use super.Blat as Gnat;

pub let plus = fun plus(-a: Gnat, b: Gnat): Gnat {
    match a {
        .O => b,
        .S(a') => Gnat.S(plus(a', b)),
    }
};
