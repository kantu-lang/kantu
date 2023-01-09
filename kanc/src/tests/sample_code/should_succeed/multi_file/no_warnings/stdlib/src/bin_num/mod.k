pub type Bit {
    .Zero: Bit,
    .One: Bit,
}

pub type PosInt {
    .One: PosInt,
    .Cons(left: PosInt, right: Bit): PosInt,
}

pub type Nat {
    .Zero: Nat,
    .Pos(n: PosInt): Nat,
}

pub type Int {
    .Neg(n: PosInt): Int,
    .Zero: Int,
    .Pos(n: PosInt): Int,
}

pub mod ops;
