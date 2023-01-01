mod nat;
use nat.Blat;

let plus = fun plus(-a: Blat, b: Blat): Blat {
    match a {
        .O => b,
        .S(a') => Blat.S(plus(a', b)),
    }
};
