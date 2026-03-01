//! Definitions of common HIR node types describing various instructions and
//! language constructs.
use crate::hir::arena;
use crate::hir::context::HirFunctionContext;
use haikulang_parser::span::Span;

/// Holder of a literal value.
#[derive(Clone, Debug)]
pub struct HirLiteral {
    pub kind: HirLiteralKind,
    pub span: Span,
}

/// The variant of the literal value.
#[derive(Clone, Debug)]
pub enum HirLiteralKind {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(HirStringId),
}

/// The type for string literals or a variable name.
pub type HirString = String;

/// Reference to a String literal that is interned.
pub type HirStringId = arena::Id<HirString>;

/// Representation of a local variable.
#[derive(Clone, Debug)]
pub struct HirVariable {
    pub name: HirStringId,
    pub location: Span,
}

/// Reference to a collected local variable.
pub type HirVariableId = arena::Id<HirVariable>;

/// Representation of an expression.
#[derive(Clone, Debug)]
pub struct HirExpr {
    pub kind: HirExprKind,
    pub span: Span,
}

/// Reference to an expression to evaluate.
pub type HirExprId = arena::Id<HirExpr>;

/// The variant of an expression.
#[derive(Clone, Debug)]
pub enum HirExprKind {
    LoadLiteral(HirLiteral),
    LoadVariable(HirVariableId),
    BinaryOp {
        left: HirExprId,
        op: HirExprBinaryOp,
        right: HirExprId,
    },
    UnaryOp {
        op: HirExprUnaryOp,
        value: HirExprId,
    },

    // Something probably in an outside scope, since it is definitely not in this scope.
    Unresolved(HirString),
}

/// Operators that can be used in binary expressions.
#[derive(Clone, Debug)]
pub enum HirExprBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    BinaryShl,
    BinaryShr,
    BoolAnd,
    BoolOr,
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

/// Operators that can be used in unary expressions.
#[derive(Clone, Debug)]
pub enum HirExprUnaryOp {
    Negate,
    Not,
    Invert,
}

/// Representation of a statement within a function.
#[derive(Clone, Debug)]
pub struct HirStatement {
    pub kind: HirStatementKind,
    pub span: Span,
}

/// Reference to a statement to evaluate.
pub type HirStatementId = arena::Id<HirStatement>;

/// The variant of a statement.
#[derive(Clone, Debug)]
pub enum HirStatementKind {
    Empty,
    VarDecl {
        variable: HirVariableId,
        expr: Option<HirExprId>,
    },
    Expr(HirExprId),
    Return(Option<HirExprId>),
    Continue,
    Break,
    If {
        condition: HirExprId,
        then: HirStatementId,
        otherwise: Option<HirStatementId>,
    },
    While {
        condition: HirExprId,
        body: HirStatementId,
    },
    Block(HirBlock),
}

/// Representation of a function prototype which we have not yet lowered.
#[derive(Debug)]
pub struct HirFunctionHeader {
    pub name: HirStringId,
}

/// Representation of a function body for a lowered function prototype.
#[derive(Debug)]
pub struct HirFunctionData<'context> {
    pub context: HirFunctionContext<'context>,
    pub header: HirFunctionHeader,
    pub root_statement: HirStatementId,
}

/// Representation of a block body.
pub type HirBlock = Vec<HirStatementId>;
