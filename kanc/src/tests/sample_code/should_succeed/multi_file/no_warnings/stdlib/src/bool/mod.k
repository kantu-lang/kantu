pub type Bool {
    .true: Bool,
    .false: Bool,
}

pub type BoolTrue(b: Bool) {
    .refl: BoolTrue(Bool.true),
}

pub mod ops;
pub mod theorems;
