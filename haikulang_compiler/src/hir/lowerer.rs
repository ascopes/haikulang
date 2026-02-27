use crate::hir::arena::Arena;
use crate::hir::context::{HirFunctionContext, HirModuleContext};
use crate::hir::nodes::*;
use crate::hir::sym::SymbolTable;
use haikulang_parser::ast::expr::Expr;
use haikulang_parser::ast::func::FunctionDecl;
use haikulang_parser::ast::stmt::{
    BlockStatement, IfStatement, ReturnStatement, Statement, VarDeclStatement, WhileStatement,
};
use haikulang_parser::span::Span;

impl<'module> HirFunctionContext<'module> {
    pub fn new(module_context: &'module mut HirModuleContext) -> Self {
        Self {
            module_context,
            symbol_table: SymbolTable::new(),
            expr_arena: Arena::new(),
            statement_arena: Arena::new(),
            variable_arena: Arena::new(),
        }
    }

    pub fn get_string(&self, id: HirStringId) -> &HirString {
        self.module_context.string_interner.get(id)
    }

    pub fn get_expr(&self, id: HirExprId) -> &HirExpr {
        &self.expr_arena[id]
    }

    pub fn get_statement(&self, id: HirStatementId) -> &HirStatement {
        &self.statement_arena[id]
    }

    pub fn get_variable(&self, id: HirVariableId) -> &HirVariable {
        &self.variable_arena[id]
    }

    pub fn lower_function(
        mut self,
        function_header: HirFunctionHeader,
        function_decl: &FunctionDecl,
    ) -> HirFunctionData<'module> {
        self.symbol_table.push();

        // Declare our parameters.
        for param in &function_decl.parameters.value() {
            let param_name = param.value().name.value().value;
            let param_name_id = self.module_context.string_interner.intern(param_name);
            let variable = HirVariable {
                name: param_name_id,
                location: param.span(),
            };
            let variable_id = self.variable_arena.alloc(variable);
            self.symbol_table.declare(param_name_id, variable_id);
        }

        let body = self.lower_statement(&function_decl.body.value(), function_decl.body.span());

        self.symbol_table.pop();

        HirFunctionData {
            context: self,
            header: function_header,
            root_statement: body,
        }
    }

    fn lower_statement(&mut self, statement: &Statement, span: Span) -> HirStatementId {
        let kind = match statement {
            Statement::Empty => HirStatementKind::Empty,
            Statement::Expr(expr_stmt) => HirStatementKind::Expr(self.lower_expr(expr_stmt, span)),
            Statement::VarDecl(var_decl_stmt) => self.lower_var_decl_statement(var_decl_stmt, span),
            Statement::Break => HirStatementKind::Break,
            Statement::Continue => HirStatementKind::Continue,
            Statement::Return(return_stmt) => self.lower_return_statement(return_stmt),
            Statement::If(if_stmt) => self.lower_if_statement(if_stmt),
            Statement::While(while_stmt) => self.lower_while_statement(while_stmt),
            Statement::Block(block_stmt) => self.lower_block_statement(block_stmt),
        };

        let stmt = HirStatement { kind, span };
        self.statement_arena.alloc(stmt)
    }

    fn lower_var_decl_statement(
        &mut self,
        var_decl_statement: &VarDeclStatement,
        span: Span,
    ) -> HirStatementKind {
        let identifier = var_decl_statement.identifier.value().value;
        let identifier_id = self.module_context.string_interner.intern(identifier);
        let variable = HirVariable {
            name: identifier_id,
            location: span,
        };
        let variable_id = self.variable_arena.alloc(variable);
        self.symbol_table.declare(identifier_id, variable_id);

        let expr = var_decl_statement
            .expr
            .as_ref()
            .map(|expr| self.lower_expr(&expr.value(), expr.span()));
        HirStatementKind::VarDecl {
            variable: variable_id,
            expr,
        }
    }

    fn lower_return_statement(&mut self, return_statement: &ReturnStatement) -> HirStatementKind {
        let expr = return_statement
            .expr
            .as_ref()
            .map(|expr| self.lower_expr(&expr.value(), expr.span()));
        HirStatementKind::Return(expr)
    }

    fn lower_if_statement(&mut self, if_statement: &IfStatement) -> HirStatementKind {
        let condition = self.lower_expr(
            &if_statement.condition.value(),
            if_statement.condition.span(),
        );
        let then = self.lower_statement(&if_statement.body.value(), if_statement.body.span());
        let otherwise = if_statement
            .otherwise
            .as_ref()
            .map(|otherwise| self.lower_statement(&otherwise.value(), otherwise.span()));
        HirStatementKind::If {
            condition,
            then,
            otherwise,
        }
    }

    fn lower_while_statement(&mut self, while_statement: &WhileStatement) -> HirStatementKind {
        let condition = self.lower_expr(
            &while_statement.condition.value(),
            while_statement.condition.span(),
        );
        let body = self.lower_statement(&while_statement.body.value(), while_statement.body.span());
        HirStatementKind::While { condition, body }
    }

    fn lower_block_statement(&mut self, block_statement: &BlockStatement) -> HirStatementKind {
        self.symbol_table.push();

        let mut statements: Vec<HirStatementId> = Vec::new();
        for statement in &block_statement.statements {
            statements.push(self.lower_statement(&statement.value(), statement.span()))
        }

        self.symbol_table.pop();

        HirStatementKind::Block(statements)
    }

    fn lower_expr(&mut self, expr: &Expr, span: Span) -> HirExprId {
        let kind = match expr {
            Expr::Binary(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Assignment(_) => todo!(),
            Expr::MemberAccess(_) => todo!(),
            Expr::Index(_) => todo!(),
            Expr::FunctionCall(_) => todo!(),
            Expr::Float(_) => todo!(),
            Expr::Int(_) => todo!(),
            Expr::Bool(_) => todo!(),
            Expr::String(_) => todo!(),
            Expr::IdentifierPath(_) => todo!(),
        };

        let expr = HirExpr { kind, span };
        self.expr_arena.alloc(expr)
    }
}
