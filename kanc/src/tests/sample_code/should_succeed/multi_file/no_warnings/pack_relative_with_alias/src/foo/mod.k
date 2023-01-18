use mod as boo;

mod bar;
use bar.*;
use Nat.*;

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .o => b,
        .s(a') => s(plus(a', b)),
    }
};
