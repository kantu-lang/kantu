mod nat;
pub use nat.Nat;

use Nat.*;

mod ops;

// TODO: Use more wildcards once we implement visibility
// (it currently causes a name clash, since even private items
// are imported.)

pub let(*) factorial = fun factorial(-n: Nat): nat.Nat {
    match n {
        .O => pack.nat.Nat.S(O),
        .S(n') => mod.ops.mult(n, factorial(n')),
    }
};
