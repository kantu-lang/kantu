use crate::internal_prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub statements: Vec<FileStatement>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileStatement {
    Type(TypeStatement),
    Let(LetStatement),
    Def(DefStatement),
    Use(UseStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub visibility_clause: Option<VisibilityClause>,
    pub type_kw_position: TextPosition,
    pub name: Token,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VisibilityClause {
    pub pub_kw_position: TextPosition,
    pub scope_modifier: Option<ParenthesizedScopeModifier>,
}
