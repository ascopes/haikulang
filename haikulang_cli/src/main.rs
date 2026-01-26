use haikulang_parser::lexer::*;
use haikulang_parser::parser::*;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();

    loop {
        print!(">>> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let tokens = TokenStream::new(input.as_str());

        let mut parser = Parser::new(tokens);

        let output = match parser.parse_expr() {
            Ok(ast) => visit(ast.value()),
            Err(err) => panic!("{:?}", err),
        };

        println!("{}\n", output);
    }
}

fn visit(ast: AstNode) -> f64 {
    match ast {
        AstNode::BinaryOp(left, op, right) => visit_binary_op(left.value(), op, right.value()),
        AstNode::UnaryOp(op, right) => visit_unary_op(op, right.value()),
        AstNode::Float(value) => value,
        AstNode::Int(value) => value as f64,
        other => panic!("Unsupported AST node {:?}", other),
    }
}

fn visit_binary_op(left: AstNode, op: BinaryOp, right: AstNode) -> f64 {
    match op {
        BinaryOp::Add => visit(left) + visit(right),
        BinaryOp::Sub => visit(left) - visit(right),
        BinaryOp::Mul => visit(left) * visit(right),
        BinaryOp::Div => visit(left) / visit(right),
        BinaryOp::Mod => visit(left) % visit(right),
        BinaryOp::Pow => f64::powf(visit(left), visit(right)),
        BinaryOp::BinaryAnd => (visit(left) as i64 & visit(right) as i64) as f64,
        BinaryOp::BinaryOr => (visit(left) as i64 | visit(right) as i64) as f64,
        BinaryOp::BinaryXor => (visit(left) as i64 ^ visit(right) as i64) as f64,
        BinaryOp::BinaryShl => ((visit(left) as i64) << (visit(right) as i64)) as f64,
        BinaryOp::BinaryShr => ((visit(left) as i64) >> (visit(right) as i64)) as f64,
        other => panic!("Unsupported binary op {:?}", other),
    }
}

fn visit_unary_op(op: UnaryOp, right: AstNode) -> f64 {
    match op {
        UnaryOp::Plus => visit(right),
        UnaryOp::Minus => -visit(right),
        UnaryOp::Invert => !(visit(right) as i64) as f64,
        other => panic!("Unsupported unary op {:?}", other),
    }
}
