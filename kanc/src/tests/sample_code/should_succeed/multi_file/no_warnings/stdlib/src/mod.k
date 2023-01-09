pub type Empty {}

pub let(*) Not = fun _(T: Type): Type {
    forall(_: T) { Empty }
};

pub type Unit {
    .c: Unit,
}

pub type And(A: Type, B: Type) {
    .c(A: Type, a: A, B: Type, b: B): And(A, B),
}

pub type Or(A: Type, B: Type) {
    .inl(A: Type, a: A, B: Type): Or(A, B),
    .inr(B: Type, b: B, A: Type): Or(A, B),
}

pub let(*) Iff = fun _(A: Type, B: Type): Type {
    And(forall(_: A) { B }, forall(_: B) { A })
};

pub mod eq;

pub let(*) identity = fun _(T: Type, t: T): T {
    t
};

pub let(*) ascribe = identity;

pub mod bool;
pub mod simp_nat;
pub mod bin_num;
pub mod list;
pub mod option;
pub mod string;
