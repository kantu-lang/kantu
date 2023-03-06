use super.*;

pub type Bit {
    zero: Bit,
    one: Bit,
}

mod unsized;
pub use unsized.*;

pub mod bit;
pub mod pos;
pub mod nat;
pub mod int;

mod sized;
pub use sized.*;
