mod nat;
use nat.Nat as Blat;

mod ops;
pub use ops as blops;

use Blat.*;
use O as Z;

pub let mult = fun mult(-a: Blat, b: Blat): Blat {
    match a {
        .O => Z,
        .S(a') => blops.plus(b, mult(a', b)),
    }
};
