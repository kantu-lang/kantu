use super.*;

pub type Eq(T: Type, left: T, right: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}