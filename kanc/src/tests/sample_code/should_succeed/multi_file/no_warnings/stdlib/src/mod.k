pub type Equal(T: Type, a: T, b: T) {
    refl(T: Type, a: T): Equal(T, a, a),
}

pub mod bool;
use bool.prelude.*;

pub mod pair;
use pair.Pair;

pub mod num;
