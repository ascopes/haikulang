use crate::ast::expr::{
    AssignmentExpr, BinaryExpr, Expr, FunctionCallExpr, IndexExpr, MemberAccessExpr, UnaryExpr,
};
use crate::ast::func::{ExternFunctionDecl, FunctionDecl, ParameterDecl};
use crate::ast::ident::{Identifier, IdentifierPath};
use crate::ast::stmt::{
    BlockStatement, IfStatement, ReturnStatement, Statement, VarDeclStatement, WhileStatement,
};
use crate::ast::structs::{StructDecl, StructMemberDecl};
use crate::ast::unit::{CompilationUnit, CompilationUnitMember, UseDecl};
use crate::lexer::token::{FloatLit, IntLit, StrLit};
use crate::span::Span;

pub type VisitorResult<R, E> = Result<R, E>;

pub trait Visitor<R, E> {
    // Expressions
    fn visit_expr(&mut self, node: &Expr, span: Span) -> VisitorResult<R, E> {
        match node {
            Expr::Binary(inner) => self.visit_binary_expr(inner, span),
            Expr::Unary(inner) => self.visit_unary_expr(inner, span),
            Expr::Assignment(inner) => self.visit_assignment_expr(inner, span),
            Expr::MemberAccess(inner) => self.visit_member_access_expr(inner, span),
            Expr::Index(inner) => self.visit_index_expr(inner, span),
            Expr::FunctionCall(inner) => self.visit_function_call_expr(inner, span),
            Expr::Float(inner) => self.visit_float_lit(inner, span),
            Expr::Int(inner) => self.visit_int_lit(inner, span),
            Expr::Bool(inner) => self.visit_bool_lit(inner, span),
            Expr::String(inner) => self.visit_str_lit(inner, span),
            Expr::IdentifierPath(inner) => self.visit_identifier_path(inner, span),
        }
    }

    fn visit_binary_expr(&mut self, node: &BinaryExpr, span: Span) -> VisitorResult<R, E>;
    fn visit_unary_expr(&mut self, node: &UnaryExpr, span: Span) -> VisitorResult<R, E>;
    fn visit_assignment_expr(&mut self, node: &AssignmentExpr, span: Span) -> VisitorResult<R, E>;
    fn visit_member_access_expr(
        &mut self,
        node: &MemberAccessExpr,
        span: Span,
    ) -> VisitorResult<R, E>;
    fn visit_index_expr(&mut self, node: &IndexExpr, span: Span) -> VisitorResult<R, E>;
    fn visit_function_call_expr(
        &mut self,
        node: &FunctionCallExpr,
        span: Span,
    ) -> VisitorResult<R, E>;
    fn visit_float_lit(&mut self, node: &FloatLit, span: Span) -> VisitorResult<R, E>;
    fn visit_int_lit(&mut self, node: &IntLit, span: Span) -> VisitorResult<R, E>;
    fn visit_bool_lit(&mut self, node: &bool, span: Span) -> VisitorResult<R, E>;
    fn visit_str_lit(&mut self, node: &StrLit, span: Span) -> VisitorResult<R, E>;

    // Functions
    fn visit_extern_function_decl(
        &mut self,
        node: &ExternFunctionDecl,
        span: Span,
    ) -> VisitorResult<R, E>;
    fn visit_function_decl(&mut self, node: &FunctionDecl, span: Span) -> VisitorResult<R, E>;
    fn visit_parameter_decl(&mut self, node: &ParameterDecl, span: Span) -> VisitorResult<R, E>;

    // Identifiers
    fn visit_identifier_path(&mut self, node: &IdentifierPath, span: Span) -> VisitorResult<R, E>;
    fn visit_identifier(&mut self, node: &Identifier, span: Span) -> VisitorResult<R, E>;

    // Statements
    fn visit_statement(&mut self, node: &Statement, span: Span) -> VisitorResult<R, E> {
        match node {
            Statement::Empty => self.visit_empty_statement(&(), span),
            Statement::Expr(inner) => self.visit_expr(inner, span),
            Statement::VarDecl(inner) => self.visit_var_decl_statement(inner, span),
            Statement::If(inner) => self.visit_if_statement(inner, span),
            Statement::While(inner) => self.visit_while_statement(inner, span),
            Statement::Block(inner) => self.visit_block_statement(inner, span),
            Statement::Break => self.visit_break_statement(&(), span),
            Statement::Continue => self.visit_continue_statement(&(), span),
            Statement::Return(inner) => self.visit_return_statement(inner, span),
        }
    }

    fn visit_empty_statement(&mut self, node: &(), span: Span) -> VisitorResult<R, E>;
    fn visit_var_decl_statement(
        &mut self,
        node: &VarDeclStatement,
        span: Span,
    ) -> VisitorResult<R, E>;
    fn visit_if_statement(&mut self, node: &IfStatement, span: Span) -> VisitorResult<R, E>;
    fn visit_while_statement(&mut self, node: &WhileStatement, span: Span) -> VisitorResult<R, E>;
    fn visit_block_statement(&mut self, node: &BlockStatement, span: Span) -> VisitorResult<R, E>;
    fn visit_break_statement(&mut self, node: &(), span: Span) -> VisitorResult<R, E>;
    fn visit_continue_statement(&mut self, node: &(), span: Span) -> VisitorResult<R, E>;
    fn visit_return_statement(&mut self, node: &ReturnStatement, span: Span)
    -> VisitorResult<R, E>;

    // Structs
    fn visit_struct_decl(&mut self, node: &StructDecl, span: Span) -> VisitorResult<R, E>;
    fn visit_struct_member_decl(
        &mut self,
        node: &StructMemberDecl,
        span: Span,
    ) -> VisitorResult<R, E>;

    // Compilation units
    fn visit_compilation_unit(&mut self, node: &CompilationUnit, span: Span)
    -> VisitorResult<R, E>;
    fn visit_compilation_unit_member(
        &mut self,
        node: &CompilationUnitMember,
        span: Span,
    ) -> VisitorResult<R, E> {
        match node {
            CompilationUnitMember::Use(inner) => self.visit_use_decl(inner, span),
            CompilationUnitMember::ExternFunction(inner) => {
                self.visit_extern_function_decl(inner, span)
            }
            CompilationUnitMember::Function(inner) => self.visit_function_decl(inner, span),
            CompilationUnitMember::Struct(inner) => self.visit_struct_decl(inner, span),
        }
    }
    fn visit_use_decl(&mut self, node: &UseDecl, span: Span) -> VisitorResult<R, E>;
}
