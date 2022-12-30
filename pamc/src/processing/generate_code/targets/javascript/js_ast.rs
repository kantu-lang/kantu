#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Const(ConstStatement),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstStatement {
    pub name: ValidJsIdentifierName,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Literal(Literal),
    Identifier(ValidJsIdentifierName),
    Call(Box<Call>),
    New(Box<Call>),
    Function(Box<Function>),
    BinaryOp(Box<BinaryOp>),
    Dot(Box<Dot>),
    Ternary(Box<Ternary>),
    Array(Box<Array>),
    Object(Box<Object>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValidJsIdentifierName(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    Boolean(bool),
    Number(i32),
    String(JsStringLiteral),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct JsStringLiteral {
    pub unescaped: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    pub name: ValidJsIdentifierName,
    pub params: Params,
    pub body: Vec<FunctionStatement>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Params {
    Standard(Vec<ValidJsIdentifierName>),
    DestructuredSingleton(Vec<ObjectDestructureEntry>),
}

impl Params {
    pub fn is_empty(&self) -> bool {
        match self {
            Params::Standard(params) => params.is_empty(),
            Params::DestructuredSingleton(entries) => entries.is_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectDestructureEntry {
    pub in_name: ValidJsIdentifierName,
    pub out_name: ValidJsIdentifierName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FunctionStatement {
    Const(ConstStatement),
    If(IfStatement),
    Return(Expression),
    Throw(Expression),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<FunctionStatement>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryOp {
    pub op: BinaryOpKind,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    TripleEqual,
    Plus,
    Index,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dot {
    pub left: Expression,
    pub right: ValidJsIdentifierName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ternary {
    pub condition: Expression,
    pub true_body: Expression,
    pub false_body: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array {
    pub items: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Object {
    pub entries: Vec<ObjectEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectEntry {
    pub key: ValidJsIdentifierName,
    pub value: Expression,
}
