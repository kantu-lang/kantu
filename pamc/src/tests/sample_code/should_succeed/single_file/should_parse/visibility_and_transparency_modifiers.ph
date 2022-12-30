use mod_a;
pub use mod_b;
pub(*) use mod_c;
pub(mod) use mod_d;
pub(super) use mod_e;
pub(super2) use mod_f;
pub(super8) use mod_g;
pub(pack) use mod_h;
pub(pack.some) use mod_i;
pub(pack.some.module) use mod_j;

mod mod_a;
pub mod mod_b;
pub(*) mod mod_c;
pub(mod) mod mod_d;
pub(super) mod mod_e;
pub(super2) mod mod_f;
pub(super8) mod mod_g;
pub(pack) mod mod_h;
pub(pack.some) mod mod_i;
pub(pack.some.module) mod mod_j;

type A {}
pub type B {}
pub(*) type C {}
pub(mod) type D {}
pub(super) type E {}
pub(super2) type F {}
pub(super8) type G {}
pub(pack) type H {}
pub(pack.some) type I {}
pub(pack.some.module) type J {}

type Unit {
    .C: Unit,
}

let a = Unit.C;
pub let b = Unit.C;
pub(*) let c = Unit.C;
pub(mod) let d = Unit.C;
pub(super) let e = Unit.C;
pub(super2) let f = Unit.C;
pub(super8) let g = Unit.C;
pub(pack) let h = Unit.C;
pub(pack.some) let i = Unit.C;
pub(pack.some.module) let j = Unit.C;

let a2 = Unit.C;
let(*) b2 = Unit.C;
let(mod) c2 = Unit.C;
let(super) d2 = Unit.C;
let(super2) e2 = Unit.C;
let(super8) f2 = Unit.C;
let(pack) g2 = Unit.C;
let(pack.some) h2 = Unit.C;
let(pack.some.module) i2 = Unit.C;

let a3 = Unit.C;
pub let b3 = Unit.C;
pub(pack.some.module) let c3 = Unit.C;
let(pack.some.module) d3 = Unit.C;
pub let(pack.some.module) e3 = Unit.C;
pub(pack.some.module) let(pack.some.module) f3 = Unit.C;