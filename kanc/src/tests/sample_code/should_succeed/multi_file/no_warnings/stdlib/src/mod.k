pub type Equal(T: Type, a: T, b: T) {
    refl(T: Type, a: T): Equal(T, a, a),
}

pub mod bool;
use bool.prelude.*;

pub mod prod;
use prod.Prod;

pub mod sum;
use sum.Sum;

pub mod opt;
use opt.Opt;

pub mod list;
use list.List;

pub mod num;

pub mod time;
