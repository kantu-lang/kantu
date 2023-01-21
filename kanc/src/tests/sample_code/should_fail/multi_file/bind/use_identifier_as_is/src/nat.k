pub type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

use Nat as Blat;
pub use Blat;
