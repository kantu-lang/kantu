use super.*;

pub type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}
