mod nat;
pub use nat.Nat;

use Nat.*;

mod ops;
use ops.*;

pub let(*) factorial = fun factorial(-n: Nat): nat.Nat {
    match n {
        .O => pack.nat.Nat.S(O),
        .S(n') => mult(n, factorial(n')),
    }
};
