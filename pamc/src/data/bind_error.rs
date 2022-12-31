use crate::data::{simplified_ast as unbound, simplified_ast::IdentifierName, FileId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindError {
    NameNotFound(NameNotFoundError),
    NameClash(NameClashError),
    ExpectedTermButNameRefersToMod(ExpectedTermButNameRefersToModError),
    CannotUselesslyImportItemAsItself(CannotUselesslyImportItemAsItselfError),
    ModFileNotFound(ModFileNotFoundError),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameClashError {
    pub name: IdentifierName,
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
    Mod(FileId),
    WildcardImport(unbound::UseWildcardStatement),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CannotUselesslyImportItemAsItselfError {
    pub use_statement: unbound::UseSingleStatement,
}
impl From<CannotUselesslyImportItemAsItselfError> for BindError {
    fn from(error: CannotUselesslyImportItemAsItselfError) -> Self {
        Self::CannotUselesslyImportItemAsItself(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModFileNotFoundError {
    pub mod_name: unbound::Identifier,
}
impl From<ModFileNotFoundError> for BindError {
    fn from(error: ModFileNotFoundError) -> Self {
        Self::ModFileNotFound(error)
    }
}
