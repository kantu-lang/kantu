mod nat;
use nat.Nat as Blat;

pub mod ops;
pub use ops as blops;

use Blat.*;
use o as z;

pub let mult = fun mult(-a: Blat, b: Blat): Blat {
    match a {
        .o => z,
        .s(a') => blops.plus(b, mult(a', b)),
    }
};
