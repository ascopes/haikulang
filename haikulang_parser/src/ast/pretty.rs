use crate::ast::expr::{
    AssignmentExpr, BinaryExpr, FunctionCallExpr, IndexExpr, MemberAccessExpr, UnaryExpr,
};
use crate::ast::func::{ExternFunctionDecl, FunctionDecl, ParameterDecl};
use crate::ast::ident::{Identifier, IdentifierPath};
use crate::ast::stmt::{
    BlockStatement, IfStatement, ReturnStatement, VarDeclStatement, WhileStatement,
};
use crate::ast::structs::{StructDecl, StructMemberDecl};
use crate::ast::unit::{CompilationUnit, UseDecl};
use crate::ast::visitor::{Visitor, VisitorResult};
use crate::lexer::token::{FloatLit, IntLit, StrLit};
use crate::span::{Span, Spanned};
use std::fmt;

const INDENT: usize = 4;

type PrettyPrinterResult = VisitorResult<(), fmt::Error>;

pub struct PrettyPrinterVisitor<'write> {
    printer: Box<dyn fmt::Write + 'write>,
    start_of_line: bool,
    level: usize,
}

impl<'write> PrettyPrinterVisitor<'write> {
    pub fn new(printer: impl fmt::Write + 'write) -> Self {
        Self {
            printer: Box::from(printer),
            start_of_line: true,
            level: 0,
        }
    }

    fn block<F>(&mut self, name: impl ToString, span: Span, func: F) -> PrettyPrinterResult
    where
        F: Fn(&mut Self) -> PrettyPrinterResult,
    {
        self.write_line(format!("{}({}) {{", name.to_string(), span))?;
        self.level += 1;
        let result = func(self);
        self.level -= 1;
        self.write_line("}")?;
        result
    }

    fn write_prefix(&mut self, content: impl ToString) -> PrettyPrinterResult {
        if self.start_of_line {
            for _ in 0..=self.level {
                // Indent
                for _ in 0..INDENT {
                    self.printer.write_char(' ')?
                }
            }
        }
        self.start_of_line = false;
        self.printer.write_str(&content.to_string())?;
        Ok(())
    }

    fn write_line(&mut self, content: impl ToString) -> PrettyPrinterResult {
        self.write_prefix(content)?;
        self.printer.write_char('\n')?;
        self.start_of_line = true;
        Ok(())
    }

    fn visit_parameters(
        &mut self,
        params: Box<[Spanned<ParameterDecl>]>,
        _: Span,
    ) -> PrettyPrinterResult {
        for (i, argument) in params.iter().enumerate() {
            self.write_prefix(format!("parameter_{}: ", i))?;
            self.visit_parameter_decl(&argument.value(), argument.span())?;
        }

        Ok(())
    }

    fn visit_return_type(
        &mut self,
        return_type: &Option<Spanned<IdentifierPath>>,
    ) -> PrettyPrinterResult {
        self.write_prefix("return_type: ")?;
        if let Some(present_return_type) = return_type {
            self.visit_identifier_path(&present_return_type.value(), present_return_type.span())?;
        } else {
            self.write_line("none")?;
        }

        Ok(())
    }
}

impl<'write> Visitor<(), fmt::Error> for PrettyPrinterVisitor<'write> {
    fn visit_binary_expr(&mut self, node: &BinaryExpr, span: Span) -> PrettyPrinterResult {
        self.block("BinaryExpr", span, |visitor| {
            visitor.write_prefix("left: ")?;
            visitor.visit_expr(&node.left.value(), node.left.span())?;

            visitor.write_line(format!("op: {:?}", node.op))?;

            visitor.write_prefix("right: ")?;
            visitor.visit_expr(&node.left.value(), node.left.span())?;
            Ok(())
        })
    }

    fn visit_unary_expr(&mut self, node: &UnaryExpr, span: Span) -> PrettyPrinterResult {
        self.block("UnaryExpr", span, |visitor| {
            visitor.write_line(format!("op: {:?}", node.op))?;

            visitor.write_prefix("value: ")?;
            visitor.visit_expr(&node.value.value(), node.value.span())?;
            Ok(())
        })
    }

    fn visit_assignment_expr(&mut self, node: &AssignmentExpr, span: Span) -> PrettyPrinterResult {
        self.block("AssignmentExpr", span, |visitor| {
            visitor.write_prefix("lvalue: ")?;
            visitor.visit_expr(&node.lvalue.value(), node.lvalue.span())?;

            if let Some(op) = &node.op {
                visitor.write_line(format!("op: {:?}", op))?;
            } else {
                visitor.write_line("op: none")?;
            }

            visitor.write_prefix("rvalue: ")?;
            visitor.visit_expr(&node.rvalue.value(), node.rvalue.span())?;
            Ok(())
        })
    }

    fn visit_member_access_expr(
        &mut self,
        node: &MemberAccessExpr,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block("MemberAccessExpr", span, |visitor| {
            visitor.write_prefix("owner: ")?;
            visitor.visit_expr(&node.owner.value(), node.owner.span())?;

            visitor.write_prefix("member: ")?;
            visitor.visit_identifier(&node.member.value(), node.member.span())?;
            Ok(())
        })
    }

    fn visit_index_expr(&mut self, node: &IndexExpr, span: Span) -> PrettyPrinterResult {
        self.block("IndexExpr", span, |visitor| {
            visitor.write_prefix("owner: ")?;
            visitor.visit_expr(&node.owner.value(), node.owner.span())?;

            visitor.write_prefix("index: ")?;
            visitor.visit_expr(&node.index.value(), node.index.span())?;
            Ok(())
        })
    }

    fn visit_function_call_expr(
        &mut self,
        node: &FunctionCallExpr,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block("FunctionCallExpr", span, |visitor| {
            visitor.write_prefix("identity: ")?;
            visitor.visit_expr(&node.identity.value(), node.identity.span())?;
            for (i, argument) in node.arguments.value().iter().enumerate() {
                visitor.write_prefix(format!("argument_{}: ", i))?;
                visitor.visit_expr(&argument.value(), argument.span())?;
            }

            Ok(())
        })
    }

    fn visit_float_lit(&mut self, node: &FloatLit, span: Span) -> PrettyPrinterResult {
        self.write_line(format!("FloatLit ({}): {{ float: {} }}", span, node))
    }

    fn visit_int_lit(&mut self, node: &IntLit, span: Span) -> PrettyPrinterResult {
        self.write_line(format!("IntLit ({}): {{ value: {} }}", span, node))
    }

    fn visit_bool_lit(&mut self, node: &bool, span: Span) -> PrettyPrinterResult {
        self.write_line(format!("BoolLit ({}): {{ value: {} }}", span, node))
    }

    fn visit_str_lit(&mut self, node: &StrLit, span: Span) -> PrettyPrinterResult {
        self.write_line(format!("StrLit ({}): {{ value: {:?} }}", span, node))
    }

    fn visit_extern_function_decl(
        &mut self,
        node: &ExternFunctionDecl,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block("ExternFunctionDecl", span, |visitor| {
            visitor.write_prefix("name: ")?;
            visitor.visit_identifier(&node.name.value(), node.name.span())?;

            visitor.visit_parameters(node.parameters.value(), node.parameters.span())?;

            Ok(())
        })
    }

    fn visit_function_decl(&mut self, node: &FunctionDecl, span: Span) -> PrettyPrinterResult {
        self.block("FunctionDecl", span, |visitor| {
            visitor.write_prefix("name: ")?;
            visitor.visit_identifier(&node.name.value(), node.name.span())?;

            visitor.visit_parameters(node.parameters.value(), node.parameters.span())?;

            visitor.visit_return_type(&node.return_type)?;

            visitor.write_prefix("body: ")?;
            visitor.visit_statement(&node.body.value(), node.body.span())?;

            Ok(())
        })
    }

    fn visit_parameter_decl(&mut self, node: &ParameterDecl, span: Span) -> PrettyPrinterResult {
        self.block("ParameterDecl", span, |visitor| {
            visitor.write_prefix("name: ")?;
            visitor.visit_identifier(&node.name.value(), node.name.span())?;

            visitor.write_prefix("type_name: ")?;
            visitor.visit_identifier_path(&node.type_name.value(), node.type_name.span())?;

            Ok(())
        })
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath, span: Span) -> PrettyPrinterResult {
        self.block("IdentifierPath", span, |visitor| {
            for (i, qualifier) in node.qualifier.iter().enumerate() {
                visitor.write_prefix(format!("qualifier_{}: ", i))?;
                visitor.visit_identifier(&qualifier.value(), qualifier.span())?;
            }

            visitor.write_prefix("local_name: ")?;
            visitor.visit_identifier(&node.local_name.value(), node.local_name.span())?;

            Ok(())
        })
    }

    fn visit_identifier(&mut self, node: &Identifier, span: Span) -> PrettyPrinterResult {
        self.write_line(format!(
            "Identifier({}) {{ value: {:?} }}",
            span, node.value
        ))
    }

    fn visit_empty_statement(&mut self, _: &(), span: Span) -> PrettyPrinterResult {
        self.write_line(format!("EmptyStatement({})", span))
    }

    fn visit_var_decl_statement(
        &mut self,
        node: &VarDeclStatement,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block("VarDeclStatement", span, |visitor| {
            visitor.write_prefix("identifier: ")?;
            visitor.visit_identifier(&node.identifier.value(), node.identifier.span())?;

            visitor.write_prefix("type_name: ")?;
            if let Some(unwrapped_type_name) = &node.type_name {
                visitor.visit_identifier_path(
                    &unwrapped_type_name.value(),
                    unwrapped_type_name.span(),
                )?;
            } else {
                visitor.write_line("none")?;
            }

            visitor.write_prefix("expr: ")?;
            if let Some(unwrapped_expr) = &node.expr {
                visitor.visit_expr(&unwrapped_expr.value(), unwrapped_expr.span())?;
            } else {
                visitor.write_line("none")?;
            }

            Ok(())
        })
    }

    fn visit_if_statement(&mut self, node: &IfStatement, span: Span) -> PrettyPrinterResult {
        self.block("IfStatement", span, |visitor| {
            visitor.write_prefix("condition: ")?;
            visitor.visit_expr(&node.condition.value(), node.condition.span())?;

            visitor.write_prefix("body: ")?;
            visitor.visit_statement(&node.body.value(), node.body.span())?;

            visitor.write_prefix("otherwise: ")?;
            if let Some(unwrapped_otherwise) = &node.otherwise {
                visitor
                    .visit_statement(&unwrapped_otherwise.value(), unwrapped_otherwise.span())?;
            } else {
                visitor.write_line("none")?;
            }

            Ok(())
        })
    }

    fn visit_while_statement(&mut self, node: &WhileStatement, span: Span) -> PrettyPrinterResult {
        self.block("WhileStatement", span, |visitor| {
            visitor.write_prefix("condition: ")?;
            visitor.visit_expr(&node.condition.value(), node.condition.span())?;

            visitor.write_prefix("body: ")?;
            visitor.visit_statement(&node.body.value(), node.body.span())?;

            Ok(())
        })
    }

    fn visit_block_statement(&mut self, node: &BlockStatement, span: Span) -> PrettyPrinterResult {
        self.block("BlockStatement", span, |visitor| {
            for (i, statement) in node.statements.iter().enumerate() {
                visitor.write_prefix(format!("statement_{}: ", i))?;
                visitor.visit_statement(&statement.value(), statement.span())?;
            }

            Ok(())
        })
    }

    fn visit_break_statement(&mut self, _: &(), span: Span) -> PrettyPrinterResult {
        self.write_line(format!("BreakStatement({})", span))
    }

    fn visit_continue_statement(&mut self, _: &(), span: Span) -> PrettyPrinterResult {
        self.write_line(format!("ContinueStatement({})", span))
    }

    fn visit_return_statement(
        &mut self,
        node: &ReturnStatement,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block("ReturnStatement", span, |visitor| {
            visitor.write_prefix("expr: ")?;
            if let Some(unwrapped_expr) = &node.expr {
                visitor.visit_expr(&unwrapped_expr.value(), unwrapped_expr.span())?;
            } else {
                visitor.write_line("none")?;
            }

            Ok(())
        })
    }

    fn visit_struct_decl(&mut self, node: &StructDecl, span: Span) -> PrettyPrinterResult {
        self.block("StructDecl", span, |visitor| {
            visitor.write_prefix("identifier: ")?;
            visitor.visit_identifier(&node.identifier.value(), node.identifier.span())?;

            for (i, member) in node.members.iter().enumerate() {
                visitor.write_prefix(format!("member_{}: ", i))?;
                visitor.visit_struct_member_decl(&member.value(), member.span())?;
            }

            Ok(())
        })
    }

    fn visit_struct_member_decl(
        &mut self,
        node: &StructMemberDecl,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block("StructMemberDecl", span, |visitor| {
            visitor.write_prefix("identifier: ")?;
            visitor.visit_identifier(&node.identifier.value(), node.identifier.span())?;

            visitor.write_prefix("type_name: ")?;
            visitor.visit_identifier_path(&node.type_name.value(), node.type_name.span())?;

            Ok(())
        })
    }

    fn visit_compilation_unit(
        &mut self,
        node: &CompilationUnit,
        span: Span,
    ) -> PrettyPrinterResult {
        self.block(
            format!("CompilationUnit({:?})", node.path),
            span,
            |visitor| {
                for (i, member) in node.members.iter().enumerate() {
                    visitor.write_prefix(format!("member_{}: ", i))?;
                    visitor.visit_compilation_unit_member(&member.value(), member.span())?;
                }

                Ok(())
            },
        )
    }

    fn visit_use_decl(&mut self, node: &UseDecl, span: Span) -> PrettyPrinterResult {
        self.block("UseDecl", span, |visitor| {
            visitor.write_prefix("expr: ")?;
            visitor.visit_identifier_path(&node.path.value(), node.path.span())?;
            Ok(())
        })
    }
}
