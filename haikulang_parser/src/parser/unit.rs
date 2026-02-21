use crate::ast::unit::{CompilationUnit, CompilationUnitMember, UseDecl};
use crate::lexer::token::Token;
use crate::parser::core::{Parser, ParserResult};
use crate::parser::error::ParserError;
use crate::span::Spanned;
use std::path::Path;

impl<'src> Parser<'src> {
    // compilation_unit ::= compilation_unit_member* , EOF ;
    pub(super) fn parse_compilation_unit(&mut self, path: &Path) -> ParserResult<CompilationUnit> {
        let start = self.current()?.span();
        let mut members: Vec<Spanned<CompilationUnitMember>> = Vec::new();

        while self.current()?.value() != Token::Eof {
            members.push(self.parse_compilation_unit_member()?);
        }

        // No need to advance, we're already at EOF.
        let end = self.current()?.span();

        Ok(Spanned::new(
            CompilationUnit {
                path: path.to_path_buf(),
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                members: members.into_boxed_slice(),
            },
            start.to(end),
        ))
    }

    // compilation_unit_member ::= use_decl , SEMICOLON
    //                           | function_decl
    //                           | struct_decl
    //                           ;
    fn parse_compilation_unit_member(&mut self) -> ParserResult<CompilationUnitMember> {
        match self.current()?.value() {
            Token::Use => {
                let use_decl = self.parse_use_decl();
                self.eat(Token::Semicolon, "semicolon")?;
                use_decl
            },
            Token::Fn => {
                let func_decl = self.parse_function_decl()?;
                let span = func_decl.span();
                Ok(Spanned::new(
                    CompilationUnitMember::Function(Box::from(func_decl.value())),
                    span
                ))
            },
            Token::Struct => {
                let func_decl = self.parse_struct_decl()?;
                let span = func_decl.span();
                Ok(Spanned::new(
                    CompilationUnitMember::Struct(Box::from(func_decl.value())),
                    span
                ))
            },
            _ => Err(Spanned::new(
                ParserError::SyntaxError(
                    "expected a top-level declaration (use statement, function declaration, or struct declaration)".to_string(),
                ),
                self.current()?.span(),
            ))
        }
    }

    // use_decl ::= USE , type_name ;
    fn parse_use_decl(&mut self) -> ParserResult<CompilationUnitMember> {
        let use_token = self.eat(Token::Use, "'use' keyword")?;
        let path = self.parse_type_name()?;
        let span = use_token.span().to(path.span());

        Ok(Spanned::new(
            CompilationUnitMember::Use(Box::from(UseDecl { path })),
            span,
        ))
    }
}
