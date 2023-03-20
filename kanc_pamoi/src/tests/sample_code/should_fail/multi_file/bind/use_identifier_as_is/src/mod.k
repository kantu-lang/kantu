mod nat;
use nat.Blat;

let plus = fun plus(-a: Blat, b: Blat): Blat {
    match a {
        o => b,
        s(a') => Blat.s(plus(a', b)),
    }
};
