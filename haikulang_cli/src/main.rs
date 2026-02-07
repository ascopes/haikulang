use haikulang_parser::ast::expr::*;
use haikulang_parser::lexer::token_stream::TokenStream;
use haikulang_parser::parser::parser::Parser;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::collections::HashMap;

fn main() -> Result<(), ReadlineError> {
    let mut editor = DefaultEditor::new()?;
    let mut variables = HashMap::<String, f64>::new();

    loop {
        let line = editor.readline(">>> ")?;

        if line.is_empty() {
            println!("Variables:");
            for (k, v) in &variables {
                println!("{} = {}", k, v);
            }
            println!();
        } else {
            let tokens = TokenStream::new(line.as_str());

            let mut parser = Parser::new(tokens);

            match parser.parse_expr() {
                Ok(ast) => match visit(&mut variables, ast.value()) {
                    Ok(value) => println!("{}", value),
                    Err(err) => println!("Interpreter error: {:?}", err),
                },
                Err(err) => println!("Parser error: {:?} at {}", err.value(), err.span()),
            };
            println!();
        }
    }
}

macro_rules! fail {
    ($($arg:tt)*) => {
        Err(format!($($arg)*))
    };
}

fn visit(variables: &mut HashMap<String, f64>, ast: Expr) -> Result<f64, String> {
    match ast {
        Expr::Binary(expr) => {
            visit_binary_op(variables, expr.left.value(), expr.op, expr.right.value())
        }
        Expr::Unary(expr) => visit_unary_op(variables, expr.op, expr.value.value()),
        Expr::Assignment(expr) => {
            visit_assignment(variables, expr.lvalue.value(), expr.op, expr.rvalue.value())
        }
        Expr::Float(value) => Ok(value),
        Expr::Int(value) => Ok(value as f64),
        Expr::Identifier(identifier) => match variables.get(&identifier.to_string()) {
            Some(value) => Ok(*value),
            None => fail!("Unknown variable referenced: {:?}", identifier),
        },
        other => fail!("Unsupported AST node {:?}", other),
    }
}

fn visit_assignment(
    variables: &mut HashMap<String, f64>,
    lvalue: Expr,
    op: Option<BinaryOp>,
    rvalue: Expr,
) -> Result<f64, String> {
    match lvalue {
        Expr::Identifier(identifier) => {
            let final_rvalue = if let Some(bin_op) = op {
                visit_binary_op(variables, rvalue.clone(), bin_op, rvalue)?
            } else {
                visit(variables, rvalue)?
            };

            variables.insert(identifier.to_string(), final_rvalue);
            Ok(final_rvalue)
        }
        other => fail!("Unsupported AST node {:?}", other),
    }
}

fn visit_binary_op(
    variables: &mut HashMap<String, f64>,
    left: Expr,
    op: BinaryOp,
    right: Expr,
) -> Result<f64, String> {
    match op {
        BinaryOp::Add => Ok(visit(variables, left)? + visit(variables, right)?),
        BinaryOp::Sub => Ok(visit(variables, left)? - visit(variables, right)?),
        BinaryOp::Mul => Ok(visit(variables, left)? * visit(variables, right)?),
        BinaryOp::Div => Ok(visit(variables, left)? / visit(variables, right)?),
        BinaryOp::Mod => Ok(visit(variables, left)? % visit(variables, right)?),
        BinaryOp::Pow => Ok(f64::powf(visit(variables, left)?, visit(variables, right)?)),
        BinaryOp::BinaryAnd => {
            Ok((visit(variables, left)? as i64 & visit(variables, right)? as i64) as f64)
        }
        BinaryOp::BinaryOr => {
            Ok((visit(variables, left)? as i64 | visit(variables, right)? as i64) as f64)
        }
        BinaryOp::BinaryXor => {
            Ok((visit(variables, left)? as i64 ^ visit(variables, right)? as i64) as f64)
        }
        BinaryOp::BinaryShl => {
            Ok(((visit(variables, left)? as i64) << (visit(variables, right)? as i64)) as f64)
        }
        BinaryOp::BinaryShr => {
            Ok(((visit(variables, left)? as i64) >> (visit(variables, right)? as i64)) as f64)
        }
        other => fail!("Unsupported binary op {:?}", other),
    }
}

fn visit_unary_op(
    variables: &mut HashMap<String, f64>,
    op: UnaryOp,
    value: Expr,
) -> Result<f64, String> {
    match op {
        UnaryOp::Plus => visit(variables, value),
        UnaryOp::Minus => Ok(-visit(variables, value)?),
        UnaryOp::Invert => Ok(!(visit(variables, value)? as i64) as f64),
        other => fail!("Unsupported unary op {:?}", other),
    }
}
