use crate::data::{simplified_ast as unbound, simplified_ast::IdentifierName, FileId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindError {
    NameNotFound(NameNotFoundError),
    NameIsPrivate(NameIsPrivateError),
    CannotLeakPrivateName(CannotLeakPrivateNameError),
    NameClash(NameClashError),
    ExpectedTermButNameRefersToMod(ExpectedTermButNameRefersToModError),
    ExpectedModButNameRefersToTerm(ExpectedModButNameRefersToTermError),
    CannotUselesslyImportItemAsSelf(CannotUselesslyImportItemAsSelfError),
    ModFileNotFound(ModFileNotFoundError),
    VisibilityWasNotAtLeastAsPermissiveAsCurrentMod(
        VisibilityWasNotAtLeastAsPermissiveAsCurrentModError,
    ),
    TransparencyWasNotAtLeastAsPermissiveAsCurrentMod(
        TransparencyWasNotAtLeastAsPermissiveAsCurrentModError,
    ),
    TransparencyWasNotAtLeastAsPermissiveAsVisibility(
        TransparencyWasNotAtLeastAsPermissiveAsVisibilityError,
    ),
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

pub use crate::data::bound_ast::Visibility;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameIsPrivateError {
    pub name_component: unbound::Identifier,
    pub required_visibility: Visibility,
    pub actual_visibility: Visibility,
}
impl From<NameIsPrivateError> for BindError {
    fn from(error: NameIsPrivateError) -> Self {
        Self::NameIsPrivate(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CannotLeakPrivateNameError {
    pub name_component: unbound::Identifier,
    pub required_visibility: Visibility,
    pub actual_visibility: Visibility,
}
impl From<CannotLeakPrivateNameError> for BindError {
    fn from(error: CannotLeakPrivateNameError) -> Self {
        Self::CannotLeakPrivateName(error)
    }
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
pub struct ExpectedTermButNameRefersToModError {
    pub name_components: Vec<unbound::Identifier>,
}
impl From<ExpectedTermButNameRefersToModError> for BindError {
    fn from(error: ExpectedTermButNameRefersToModError) -> Self {
        Self::ExpectedTermButNameRefersToMod(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpectedModButNameRefersToTermError {
    pub name_components: Vec<unbound::Identifier>,
}
impl From<ExpectedModButNameRefersToTermError> for BindError {
    fn from(error: ExpectedModButNameRefersToTermError) -> Self {
        Self::ExpectedModButNameRefersToTerm(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CannotUselesslyImportItemAsSelfError {
    pub use_statement: unbound::UseSingleStatement,
}
impl From<CannotUselesslyImportItemAsSelfError> for BindError {
    fn from(error: CannotUselesslyImportItemAsSelfError) -> Self {
        Self::CannotUselesslyImportItemAsSelf(error)
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VisibilityWasNotAtLeastAsPermissiveAsCurrentModError {
    pub visibility_modifier: unbound::ParenthesizedModScopeModifier,
}
impl From<VisibilityWasNotAtLeastAsPermissiveAsCurrentModError> for BindError {
    fn from(error: VisibilityWasNotAtLeastAsPermissiveAsCurrentModError) -> Self {
        Self::VisibilityWasNotAtLeastAsPermissiveAsCurrentMod(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransparencyWasNotAtLeastAsPermissiveAsCurrentModError {
    pub transparency_modifier: unbound::ParenthesizedModScopeModifier,
}
impl From<TransparencyWasNotAtLeastAsPermissiveAsCurrentModError> for BindError {
    fn from(error: TransparencyWasNotAtLeastAsPermissiveAsCurrentModError) -> Self {
        Self::TransparencyWasNotAtLeastAsPermissiveAsCurrentMod(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransparencyWasNotAtLeastAsPermissiveAsVisibilityError {
    pub transparency_modifier: unbound::ParenthesizedModScopeModifier,
}
impl From<TransparencyWasNotAtLeastAsPermissiveAsVisibilityError> for BindError {
    fn from(error: TransparencyWasNotAtLeastAsPermissiveAsVisibilityError) -> Self {
        Self::TransparencyWasNotAtLeastAsPermissiveAsVisibility(error)
    }
}