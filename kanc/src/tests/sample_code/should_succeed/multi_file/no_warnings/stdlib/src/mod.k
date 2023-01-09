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
