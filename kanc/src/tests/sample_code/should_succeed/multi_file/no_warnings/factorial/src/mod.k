pub mod nat;
pub use nat.Nat;

use Nat.*;

mod ops;
use ops.*;

pub let(*) factorial = fun factorial(-n: Nat): nat.Nat {
    match n {
        .o => pack.nat.Nat.s(o),
        .s(n') => mult(n, factorial(n')),
    }
};
