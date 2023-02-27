pub type Bit {
    zero: Bit,
    one: Bit,
}

pub type Pos {
    one: Pos,
    extend(left: Pos, right: Bit): Pos,
}

pub type Nat {
    zero: Nat,
    pos(p: Pos): Nat,
}

pub type Int {
    neg(n: Pos): Int,
    nat(n: Nat): Int,
}

pub type Sign {
    pos: Sign,
    neg: Sign,
}

pub type Ord {
    lt: Ord,
    eq: Ord,
    gt: Ord,
}
