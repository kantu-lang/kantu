pub mod error;
pub mod options;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum YsclCollectionKind {
    Map,
    List,
}

pub mod prelude {
    pub use super::*;
    pub use super::{error::*, options::*};
}
