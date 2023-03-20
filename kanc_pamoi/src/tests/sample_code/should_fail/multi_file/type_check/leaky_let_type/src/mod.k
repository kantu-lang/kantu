type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

use Nat.*;

pub let _2 = s(s(o));
