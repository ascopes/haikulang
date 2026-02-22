use super::error::LexerError;
use super::helpers::*;
use logos::Logos;

pub type IntLit = u64;
pub type FloatLit = f64;
pub type StrLit = Box<str>;

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(error(LexerError, parse_unknown_input))]
#[logos(skip "[ \n\r\t]+")]
#[logos(utf8 = true)]
pub enum Token {
    // Used as a marker for the end of the source file within TokenStream.
    Eof,

    #[regex(r"//[^\r\n]*?[\r\n]?", callback = parse_inline_comment)]
    InlineComment(StrLit),

    #[regex(
        r#"(?x)(?ms)   # match across multiple lines
            /\*        # opening /*
            .*?        # any content, as little as possible
            \*/        # closing */
        "#,
        callback = parse_multiline_comment
    )]
    MultilineComment(StrLit),

    #[regex(r"[A-Za-z_][A-Za-z_0-9]*", callback = parse_identifier)]
    Identifier(StrLit),

    // STRING_LIT  ::= '"' , STRING_CHAR* , '"' ;
    // STRING_CHAR ::= '\\n'
    //               | '\\r'
    //               | '\\t'
    //               | '\\"'
    //               | '\\\\'
    //               | '\\u' , [A-Fa-f0-9]{4}    /* unicode escape sequence like \u123a */
    //               | any character except ascii/unicode control sequences, carriage returns, line feeds
    //               ;
    //
    // We do not directly consume this exact grammar here; we instead use something much more
    // lenient. Any checks for validity are all dealt with in the callback for this lexer so that
    // we can give helpful error messages.
    // This includes validating unicode escape sequences; checking strings are not crossing multiple
    // lines; ensuring the string has no control characters; ensuring the string literal has a
    // closing quote as the last character in the token.
    #[regex(
        r#"(?x)
            "            # Opening quote
            ([^"]|\\")*  # Any character that is not a closing quote (treat \" as an escape).
            "?           # Closing quote, optional. If we don't match it, we raise an error.
        "#, callback = parse_string
    )]
    StringLit(StrLit),

    //        INT_LIT ::= INT_BINARY_LIT
    //                  | INT_OCTAL_LIT
    //                  | INT_HEX_LIT
    //                  | INT_DEC_LIT
    //                  ;
    // INT_BINARY_LIT ::= ('0b' | '0B') , [01] , ([01_]* , [01])? ;
    //  INT_OCTAL_LIT ::= ('0o' | '0O') , [0-7] , ([0-7_]* , [0-7])? ;
    //    INT_HEX_LIT ::= ('0x' | '0X') , [0-9A-Fa-f] , ([0-9A-Fa-f_]* , [0-9A-Fa-f])? ;
    //    INT_DEC_LIT ::= [0-9] , ([0-9_]* , [0-9])? ;
    #[regex(
        r"(?x)
            ((0b|0B)[01]([01_]*[01])?)                         # INT_BINARY_LIT
            |
            ((0o|0O)[0-7]([0-7_]*[0-7])?)                      # INT_OCTAL_LIT
            |
            ((0x|0X)[0-9A-Fa-f]([0-9A-Fa-f_]*[0-9A-Fa-f])?)    # INT_HEX_LIT
            |
            ([0-9]([0-9_]*[0-9])?)                             # INT_DEC_LIT
        ",
        callback = parse_int_lit
    )]
    IntLit(IntLit),

    //           FLOAT_LIT ::= FLOAT_WHOLE_PART , FLOAT_FRACTION_PART , FLOAT_EXPONENT_PART?
    //                       | FLOAT_WHOLE_PART , FLOAT_EXPONENT_PART
    //                       ;
    //    FLOAT_WHOLE_PART ::= [0-9] , ([0-9_]* , [0-9])? ;
    // FLOAT_FRACTION_PART ::= "." , [0-9] , ([0-9_]* , [0-9])? ;
    // FLOAT_EXPONENT_PART ::= [eE] , [+-] , [0-9] , ([0-9_]* , [0-9])? ;
    #[regex(
        r"(?x)
            (
                [0-9]([0-9_]*[0-9])?                # FLOAT_WHOLE_PART
                \.[0-9]([0-9_]*[0-9])?              # FLOAT_FRACTION_PART
                ([eE][+-]?[0-9]([0-9_+]*[0-9])?)?   # FLOAT_EXPONENT_PART?
            )
            |
            (
                [0-9]([0-9_]*[0-9])?                # FLOAT_WHOLE_PART
                ([eE][+-]?[0-9]([0-9_+]*[0-9])?)    # FLOAT_EXPONENT_PART
            )
        ",
        callback = parse_float_lit
    )]
    FloatLit(FloatLit),

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("extern")]
    Extern,

    #[token("fn")]
    Fn,

    #[token("struct")]
    Struct,

    #[token("return")]
    Return,

    #[token("continue")]
    Continue,

    #[token("break")]
    Break,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("let")]
    Let,

    #[token("use")]
    Use,

    #[token(";")]
    Semicolon,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token(".")]
    Period,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token("->")]
    Arrow,

    #[token("=")]
    Assign,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("%")]
    Mod,

    #[token("**")]
    Pow,

    #[token("+=")]
    AddAssign,

    #[token("-=")]
    SubAssign,

    #[token("*=")]
    MulAssign,

    #[token("/=")]
    DivAssign,

    #[token("%=")]
    ModAssign,

    #[token("**=")]
    PowAssign,

    #[token("&")]
    BinaryAnd,

    #[token("|")]
    BinaryOr,

    #[token("^")]
    BinaryXor,

    #[token("~")]
    BinaryNot,

    #[token("<<")]
    BinaryShl,

    #[token(">>")]
    BinaryShr,

    #[token("&=")]
    BinaryAndAssign,

    #[token("|=")]
    BinaryOrAssign,

    #[token("^=")]
    BinaryXorAssign,

    #[token("<<=")]
    BinaryShlAssign,

    #[token(">>=")]
    BinaryShrAssign,

    #[token("&&")]
    BoolAnd,

    #[token("||")]
    BoolOr,

    #[token("!")]
    BoolNot,

    #[token("==")]
    Eq,

    #[token("!=")]
    NotEq,

    #[token("<")]
    Less,

    #[token("<=")]
    LessEq,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterEq,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(                   "//",                "" ; "empty comment")]
    #[test_case(                  "// ",               " " ; "empty comment, trailing whitespace")]
    #[test_case( "// foo bar baz\nblah",  " foo bar baz\n" ; "basic inline comment")]
    #[test_case(       "// foo bar baz",    " foo bar baz" ; "inline comment at EOF")]
    #[test_case("// foo bar baz \nblah", " foo bar baz \n" ; "basic inline comment, trailing whitespace")]
    #[test_case(      "// foo bar baz ",   " foo bar baz " ; "inline comment at EOF, trailing whitespace")]
    fn inline_comments_parse_as_expected(input: &str, expected_content: &str) {
        // Given
        let mut lexer = Token::lexer(input);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        match token {
            Token::InlineComment(str) => assert_eq!(expected_content, str.as_ref()),
            other => panic!("expected InlineComment, got {:?}", other),
        }
    }

    #[test_case(                    "/**/",                "" ; "empty comment")]
    #[test_case(                   "/* */",               " " ; "blank comment")]
    #[test_case(             "/*foo bar*/",         "foo bar" ; "simple comment")]
    #[test_case(           "/* foo bar */",       " foo bar " ; "simple comment with leading and trailing whitespace")]
    #[test_case(     "/*\n foo\n bar\n */", "\n foo\n bar\n " ; "multi-line comment")]
    fn multiline_comments_parse_as_expected(input: &str, expected_content: &str) {
        // Given
        let mut lexer = Token::lexer(input);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        match token {
            Token::MultilineComment(str) => assert_eq!(expected_content, str.as_ref()),
            other => panic!("expected MultilineComment, got {:?}", other),
        }
    }

    #[test_case(                   "i" ; "single-character lowercase identifier")]
    #[test_case(                 "foo" ; "multi-character lowercase identifier")]
    #[test_case(                   "I" ; "single-character uppercase identifier")]
    #[test_case(                 "FOO" ; "multi-character uppercase identifier")]
    #[test_case(         "array_deque" ; "snake-case identifier")]
    #[test_case(          "arrayDeque" ; "camel-case identifier")]
    #[test_case(          "ArrayDeque" ; "pascal-case identifier")]
    #[test_case(                   "_" ; "single-character underscore identifier")]
    #[test_case(              "_state" ; "lowercase identifier with leading underscore")]
    #[test_case(              "state_" ; "lowercase identifier with trailing underscore")]
    #[test_case(               "_JAZZ" ; "uppercase identifier with leading underscore")]
    #[test_case(               "JAZZ_" ; "uppercase identifier with trailing underscore")]
    #[test_case("many__under___scores" ; "repeated underscores")]
    #[test_case(              "http11" ; "lowercase with trailing numbers")]
    #[test_case(               "HTTP2" ; "uppercase with trailing numbers")]
    #[test_case(        "http3session" ; "numbers in the middle of identifiers")]
    fn identifiers_are_parsed_correctly(identifier: &str) {
        // Given
        let mut lexer = Token::lexer(identifier);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        if let Token::Identifier(boxed_str) = token {
            assert_eq!(&*boxed_str, identifier);
        } else {
            panic!("expected identifier, got {:?}", token);
        }
    }

    #[test_case(    "true",              Token::True ; "true keyword")]
    #[test_case(   "false",             Token::False ; "false keyword")]
    #[test_case(  "extern",            Token::Extern ; "extern keyword")]
    #[test_case(      "fn",                Token::Fn ; "fn keyword")]
    #[test_case(  "struct",            Token::Struct ; "struct keyword")]
    #[test_case(  "return",            Token::Return ; "return keyword")]
    #[test_case("continue",          Token::Continue ; "continue keyword")]
    #[test_case(   "break",             Token::Break ; "break keyword")]
    #[test_case(      "if",                Token::If ; "if keyword")]
    #[test_case(    "else",              Token::Else ; "else keyword")]
    #[test_case(     "for",               Token::For ; "for keyword")]
    #[test_case(   "while",             Token::While ; "while keyword")]
    #[test_case(     "let",               Token::Let ; "let keyword")]
    #[test_case(     "use",               Token::Use ; "use keyword")]
    #[test_case(       ";",         Token::Semicolon ; "semicolon")]
    #[test_case(       "{",         Token::LeftBrace ; "left brace")]
    #[test_case(       "}",        Token::RightBrace ; "right brace")]
    #[test_case(       "(",         Token::LeftParen ; "left parenthesis")]
    #[test_case(       ")",        Token::RightParen ; "right parenthesis")]
    #[test_case(       "[",       Token::LeftBracket ; "left bracket")]
    #[test_case(       "]",      Token::RightBracket ; "right bracket")]
    #[test_case(       ".",            Token::Period ; "period")]
    #[test_case(       ",",             Token::Comma ; "comma")]
    #[test_case(       ":",             Token::Colon ; "colon")]
    #[test_case(      "::",       Token::DoubleColon ; "double colon")]
    #[test_case(      "->",             Token::Arrow ; "arrow")]
    #[test_case(       "=",            Token::Assign ; "assignment operator")]
    #[test_case(       "+",               Token::Add ; "addition operator")]
    #[test_case(       "-",               Token::Sub ; "subtraction operator")]
    #[test_case(       "*",               Token::Mul ; "multiplication operator")]
    #[test_case(       "/",               Token::Div ; "division operator")]
    #[test_case(       "%",               Token::Mod ; "modulo operator")]
    #[test_case(      "**",               Token::Pow ; "power operator")]
    #[test_case(      "+=",         Token::AddAssign ; "addition assignment operator")]
    #[test_case(      "-=",         Token::SubAssign ; "subtraction assignment operator")]
    #[test_case(      "*=",         Token::MulAssign ; "multiplication assignment operator")]
    #[test_case(      "/=",         Token::DivAssign ; "division assignment operator")]
    #[test_case(      "%=",         Token::ModAssign ; "modulo assignment operator")]
    #[test_case(     "**=",         Token::PowAssign ; "power assignment operator")]
    #[test_case(       "&",         Token::BinaryAnd ; "binary and operator")]
    #[test_case(       "|",          Token::BinaryOr ; "binary or operator")]
    #[test_case(       "^",         Token::BinaryXor ; "binary xor operator")]
    #[test_case(       "~",         Token::BinaryNot ; "binary not operator")]
    #[test_case(      "<<",         Token::BinaryShl ; "binary left-shift operator")]
    #[test_case(      ">>",         Token::BinaryShr ; "binary right-shift operator")]
    #[test_case(      "&=",   Token::BinaryAndAssign ; "binary and assignment operator")]
    #[test_case(      "|=",    Token::BinaryOrAssign ; "binary or assignment operator")]
    #[test_case(      "^=",   Token::BinaryXorAssign ; "binary xor assignment operator")]
    #[test_case(     "<<=",   Token::BinaryShlAssign ; "binary left-shift assignment operator")]
    #[test_case(     ">>=",   Token::BinaryShrAssign ; "binary right-shift assignment operator")]
    #[test_case(      "&&",           Token::BoolAnd ; "boolean and operator")]
    #[test_case(      "||",            Token::BoolOr ; "boolean or operator")]
    #[test_case(      "==",                Token::Eq ; "equality operator")]
    #[test_case(      "!=",             Token::NotEq ; "inequality operator")]
    #[test_case(       "<",              Token::Less ; "less-than operator")]
    #[test_case(      "<=",            Token::LessEq ; "less-than-or-equal-to operator")]
    #[test_case(       ">",           Token::Greater ; "greater-than operator")]
    #[test_case(      ">=",         Token::GreaterEq ; "greater-than-or-equal-to operator")]
    fn basic_tokens_are_parsed_as_expected(input: &str, expected_token: Token) {
        // Given
        let mut lexer = Token::lexer(input);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        assert_eq!(expected_token, token);
    }
}
