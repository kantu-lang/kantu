use super.*;

pub type Eq(T: Type, left: T, right: T) {
    refl(T: Type, z: T): Eq(T, z, z),
}