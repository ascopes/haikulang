use crate::hir::arena::{Arena, InterningArena};
use crate::hir::nodes::{
    HirExpr, HirFunctionHeader, HirStatement, HirString, HirStringId, HirVariable, HirVariableId,
};
use crate::hir::sym::SymbolTable;
use haikulang_parser::ast::func::FunctionDecl;

#[derive(Debug)]
pub struct HirModuleContext {
    pub(crate) string_interner: InterningArena<HirString>,
    pub(crate) function_table: SymbolTable<HirStringId, HirFunctionHeader>,
    pub(crate) function_arena: Arena<HirFunctionHeader>,
}

impl HirModuleContext {
    pub fn new() -> Self {
        Self {
            string_interner: InterningArena::new(),
            function_table: SymbolTable::new(),
            function_arena: Arena::new(),
        }
    }

    // Inject the functions into this module context so we are aware of them ahead of time, allowing
    // us to refer to things further down the AST later.
    pub fn pre_scan(&mut self, functions: &[FunctionDecl]) {
        for function in functions {
            let identifier_id = self.string_interner.intern(function.name.value().value);
            let function = HirFunctionHeader {
                name: identifier_id,
            };
            self.function_table.declare(identifier_id, function);
        }
    }
}

/// Translates Haikulang ASTs to a flattened intermediate language representation that can be
/// type-checked and mapped to LLVM bytecode later. Contexts are scoped to functions.
#[derive(Debug)]
pub struct HirFunctionContext<'module> {
    pub(crate) module_context: &'module mut HirModuleContext,
    pub(crate) symbol_table: SymbolTable<HirStringId, HirVariableId>,
    pub(crate) expr_arena: Arena<HirExpr>,
    pub(crate) statement_arena: Arena<HirStatement>,
    pub(crate) variable_arena: Arena<HirVariable>,
}
