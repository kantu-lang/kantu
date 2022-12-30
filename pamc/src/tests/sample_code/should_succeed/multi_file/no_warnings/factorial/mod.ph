mod nat;
pub use nat.Nat;

mod ops;
use ops.*;

use Nat.*;

pub let(*) factorial = fun factorial(-n: Nat): Nat {
    match n {
        .O => S(O),
        .S(n') => mult(n, factorial(n')),
    }
};
