use haikulang_parser::ast::expr::{BoolLitExpr, Expr, FloatLitExpr, IntLitExpr, StrLitExpr};
use haikulang_parser::span::{Span, Spanned};
use inkwell::values::BasicValue;
use inkwell::{context, values};

pub struct Compiler<'ctx> {
    context: &'ctx context::Context,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx context::Context) -> Self {
        Self { context }
    }

    pub fn visit_expr(&self, expr: &Spanned<Expr>) -> values::BasicValueEnum<'ctx> {
        match expr.value() {
            Expr::Bool(value) => self
                .visit_bool_lit_expr(&value, expr.span())
                .as_basic_value_enum(),
            Expr::Float(value) => self
                .visit_float_lit_expr(&value, expr.span())
                .as_basic_value_enum(),
            Expr::Int(value) => self
                .visit_int_lit_expr(&value, expr.span())
                .as_basic_value_enum(),
            Expr::String(value) => self
                .visit_str_lit_expr(&value, expr.span())
                .as_basic_value_enum(),
            Expr::Binary(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Assignment(_) => todo!(),
            Expr::MemberAccess(_) => todo!(),
            Expr::Index(_) => todo!(),
            Expr::FunctionCall(_) => todo!(),
            Expr::IdentifierPath(_) => todo!(),
        }
    }

    pub fn visit_bool_lit_expr(&self, value: &BoolLitExpr, _span: Span) -> values::IntValue<'ctx> {
        self.context
            .bool_type()
            .const_int(if value.value { 1 } else { 0 }, false)
    }

    pub fn visit_float_lit_expr(
        &self,
        value: &FloatLitExpr,
        _span: Span,
    ) -> values::FloatValue<'ctx> {
        self.context.f64_type().const_float(value.value)
    }

    pub fn visit_int_lit_expr(&self, value: &IntLitExpr, _span: Span) -> values::IntValue<'ctx> {
        self.context.i64_type().const_int(value.value, true)
    }

    pub fn visit_str_lit_expr(&self, value: &StrLitExpr, _span: Span) -> values::ArrayValue<'ctx> {
        self.context.const_string(value.value.as_bytes(), true)
    }
}

pub fn test_codegen(expr: &Spanned<Expr>) {
    let ctx = context::Context::create();
    let builder = ctx.create_builder();
    let compiler = Compiler::new(&ctx);
    let expr_value = compiler.visit_expr(expr);

    let module = ctx.create_module("test");

    // int main(void)
    let main_fn_type = ctx.i64_type().fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, None);
    let main_fn_block = ctx.append_basic_block(main_fn, "entry");
    builder.position_at_end(main_fn_block);
    builder.build_return(Some(&expr_value)).unwrap();

    module.verify().unwrap();
    module.print_to_stderr();
}
