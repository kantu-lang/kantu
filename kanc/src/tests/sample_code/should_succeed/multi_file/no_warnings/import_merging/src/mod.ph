mod nat;
use nat.*;

mod export_nat_1;
use export_nat_1.Nat;

mod export_nat_2;
use export_nat_2.*;

mod export_nat_3;
use export_nat_3.NicknamedNat as Nat;

pub use nat.Nat;
