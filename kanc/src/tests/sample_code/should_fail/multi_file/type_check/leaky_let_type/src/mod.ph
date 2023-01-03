type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

use Nat.*;

pub let _2 = S(S(O));
