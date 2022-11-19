use super::*;

#[derive(Clone, Debug)]
pub enum BindError {
    CircularFileDependency(CircularFileDependencyError),
    NameNotFound(NameNotFoundError),
    InvalidDotExpressionRhs(InvalidDotExpressionRhsError),
    NameClash(NameClashError),
    DotExpressionRhsClash(DotExpressionRhsClashError),
}

#[derive(Clone, Debug)]
pub struct CircularFileDependencyError {
    pub ids: Vec<FileId>,
}
impl From<CircularFileDependencyError> for BindError {
    fn from(error: CircularFileDependencyError) -> Self {
        Self::CircularFileDependency(error)
    }
}

#[derive(Clone, Debug)]
pub struct NameClashError {
    pub old: OwnedSymbolSource,
    pub new: OwnedSymbolSource,
}
impl From<NameClashError> for BindError {
    fn from(error: NameClashError) -> Self {
        Self::NameClash(error)
    }
}

pub use super::context::OwnedSymbolSource;

#[derive(Clone, Debug)]
pub struct DotExpressionRhsClashError {
    pub old: OwnedSymbolSource,
    pub new: OwnedSymbolSource,
}
impl From<DotExpressionRhsClashError> for BindError {
    fn from(error: DotExpressionRhsClashError) -> Self {
        Self::DotExpressionRhsClash(error)
    }
}

#[derive(Clone, Debug)]
pub struct NameNotFoundError {
    pub name: Identifier,
}
impl From<NameNotFoundError> for BindError {
    fn from(error: NameNotFoundError) -> Self {
        Self::NameNotFound(error)
    }
}

#[derive(Clone, Debug)]
pub struct InvalidDotExpressionRhsError {
    pub rhs: Identifier,
}
impl From<InvalidDotExpressionRhsError> for BindError {
    fn from(error: InvalidDotExpressionRhsError) -> Self {
        Self::InvalidDotExpressionRhs(error)
    }
}
