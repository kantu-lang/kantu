use mod as boo;

mod bar;
use bar.*;
use Nat.*;

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => S(plus(a', b)),
    }
};
