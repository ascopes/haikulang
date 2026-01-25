use super::error::LexerError;
use super::helpers::*;
use logos::Logos;

pub type StringLit = String;
pub type IntLit = u64;
pub type FloatLit = f64;

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip "[ \n\r\t]+")]
#[logos(utf8 = true)]
pub enum Token {
    /*
     * Comments
     */
    #[regex(r"//[^\r\n]*?[\r\n]?", callback = parse_inline_comment)]
    InlineComment(String),

    #[regex(r"/\*[^(\*/)]*?\*/", callback = parse_multiline_comment)]
    MultilineComment(String),

    /*
     * Literals
     */
    #[regex(r"[A-Za-z_][A-Za-z_0-9]*", callback = parse_identifier)]
    Identifier(String),

    // STRING_LIT  ::= '"' , STRING_CHAR* , '"' ;
    // STRING_CHAR ::= '\\n'
    //               | '\\r'
    //               | '\\t'
    //               | '\\"'
    //               | '\\\\'
    //               | '\\u' , [A-Fa-f0-9]{4}    /* unicode escape sequence like \u123a */
    //               | any character except ascii control sequences
    //               ;
    #[regex(
        r#"(?x)
            "                           # Opening quote
            (
                \\[nrt\\"]              # \n, \r, \t, \", or \\
                |\\u[A-Fa-f0-9]{4}      # \uABCD, where ABCD is a hexadecimal sequence
                |[^\x00-\x1F]           # any character other than ascii control sequences
            )*
            "                           # Closing quote
        "#, callback = parse_string
    )]
    StringLit(StringLit),

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

    /*
     * Keywords
     */
    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("fn")]
    Fn,

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

    /*
     * Control flow syntax
     */
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

    #[token("->")]
    Arrow,

    /*
     * Assignment operators
     */
    #[token("=")]
    Assign,

    /*
     * Arithmetic operators
     */
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

    /*
     * Binary operators
     */
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

    /*
     * Boolean and comparative operators
     */
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

    Eof,
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
        assert_eq!(Token::InlineComment(expected_content.to_string()), token);
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
        assert_eq!(Token::MultilineComment(expected_content.to_string()), token);
    }

    #[test_case(    "true",              Token::True ; "true keyword")]
    #[test_case(   "false",             Token::False ; "false keyword")]
    #[test_case(      "fn",                Token::Fn ; "fn keyword")]
    #[test_case(  "return",            Token::Return ; "return keyword")]
    #[test_case("continue",          Token::Continue ; "continue keyword")]
    #[test_case(   "break",             Token::Break ; "break keyword")]
    #[test_case(      "if",                Token::If ; "if keyword")]
    #[test_case(    "else",              Token::Else ; "else keyword")]
    #[test_case(     "for",               Token::For ; "for keyword")]
    #[test_case(   "while",             Token::While ; "while keyword")]
    #[test_case(     "let",               Token::Let ; "let keyword")]
    #[test_case(       ";",         Token::Semicolon ; "semicolon")]
    #[test_case(       "{",         Token::LeftBrace ; "left brace")]
    #[test_case(       "}",        Token::RightBrace ; "right brace")]
    #[test_case(       "(",         Token::LeftParen ; "left parenthesis")]
    #[test_case(       ")",        Token::RightParen ; "right parenthesis")]
    #[test_case(       "[",       Token::LeftBracket ; "left bracket")]
    #[test_case(       "]",      Token::RightBracket ; "right bracket")]
    #[test_case(       ".",            Token::Period ; "period")]
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
