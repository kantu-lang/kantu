pub type SimpNat {
    .o: SimpNat,
    .s(n: SimpNat): SimpNat,
}

// Less than or equal to
pub type Le(a: SimpNat, b: SimpNat) {
    .Refl(a: SimpNat): Le(a, a),
    .Step(a: SimpNat, b: SimpNat, H: Le(a, b)): Le(a, SimpNat.s(b)),
}

pub mod ops;
pub mod theorems;
