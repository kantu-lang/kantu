use crate::data::{file_id::*, simplified_ast as unbound, simplified_ast::IdentifierName};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindError {
    // TODO: Make NameNotFoundError have a single name component.
    // Actually, we should probably do this for all the errors.
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
    TransparencyWasNotAtLeastAsRestrictiveAsVisibility(
        TransparencyWasNotAtLeastAsRestrictiveAsVisibilityError,
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

pub use crate::data::bound_ast::{Transparency, Visibility};

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
    pub actual_visibility: Visibility,
    pub defining_mod_id: FileId,
}
impl From<VisibilityWasNotAtLeastAsPermissiveAsCurrentModError> for BindError {
    fn from(error: VisibilityWasNotAtLeastAsPermissiveAsCurrentModError) -> Self {
        Self::VisibilityWasNotAtLeastAsPermissiveAsCurrentMod(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransparencyWasNotAtLeastAsPermissiveAsCurrentModError {
    pub transparency_modifier: unbound::ParenthesizedModScopeModifier,
    pub actual_transparency: Transparency,
    pub defining_mod_id: FileId,
}
impl From<TransparencyWasNotAtLeastAsPermissiveAsCurrentModError> for BindError {
    fn from(error: TransparencyWasNotAtLeastAsPermissiveAsCurrentModError) -> Self {
        Self::TransparencyWasNotAtLeastAsPermissiveAsCurrentMod(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransparencyWasNotAtLeastAsRestrictiveAsVisibilityError {
    pub transparency_modifier: unbound::ParenthesizedModScopeModifier,
    pub transparency: Transparency,
    pub visibility: Visibility,
}
impl From<TransparencyWasNotAtLeastAsRestrictiveAsVisibilityError> for BindError {
    fn from(error: TransparencyWasNotAtLeastAsRestrictiveAsVisibilityError) -> Self {
        Self::TransparencyWasNotAtLeastAsRestrictiveAsVisibility(error)
    }
}
