pub type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

use Nat as Blat;
pub use Blat;
