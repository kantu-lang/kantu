use crate::data::{simplified_ast as unbound, FileId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindError {
    CircularFileDependency(CircularFileDependencyError),
    NameNotFound(NameNotFoundError),
    NameClash(NameClashError),
    ExpectedTermButNameRefersToMod(ExpectedTermButNameRefersToModError),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CircularFileDependencyError {
    pub ids: Vec<FileId>,
}
impl From<CircularFileDependencyError> for BindError {
    fn from(error: CircularFileDependencyError) -> Self {
        Self::CircularFileDependency(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameClashError {
    pub old: OwnedSymbolSource,
    pub new: OwnedSymbolSource,
}
impl From<NameClashError> for BindError {
    fn from(error: NameClashError) -> Self {
        Self::NameClash(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OwnedSymbolSource {
    Identifier(unbound::Identifier),
    Builtin,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameNotFoundError {
    pub name_components: Vec<unbound::Identifier>,
}
impl From<NameNotFoundError> for BindError {
    fn from(error: NameNotFoundError) -> Self {
        Self::NameNotFound(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpectedTermButNameRefersToModError {
    pub name_components: Vec<unbound::Identifier>,
}
impl From<ExpectedTermButNameRefersToModError> for BindError {
    fn from(error: ExpectedTermButNameRefersToModError) -> Self {
        Self::ExpectedTermButNameRefersToMod(error)
    }
}
