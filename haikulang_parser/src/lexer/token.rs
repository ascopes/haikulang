use crate::error::ParserError;
use crate::lexer::helpers::*;
use crate::lexer::literals::*;
use logos::Logos;

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(error(ParserError, parse_unknown_input))]
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

    //         INT_LIT ::= RAW_INT_LIT , INT_TYPE_SUFFIX? ;
    //
    //     RAW_INT_LIT ::= INT_BINARY_LIT
    //                   | INT_OCTAL_LIT
    //                   | INT_HEX_LIT
    //                   | INT_DEC_LIT
    //                   ;
    //  INT_BINARY_LIT ::= ('0b' | '0B') , [01] , ([01_]* , [01])? ;
    //   INT_OCTAL_LIT ::= ('0o' | '0O') , [0-7] , ([0-7_]* , [0-7])? ;
    //     INT_HEX_LIT ::= ('0x' | '0X') , [0-9A-Fa-f] , ([0-9A-Fa-f_]* , [0-9A-Fa-f])? ;
    //     INT_DEC_LIT ::= [0-9] , ([0-9_]* , [0-9])? ;
    //
    // INT_TYPE_SUFFIX ::= 'i8' | 'i16' | 'i32' | 'i64'
    //                   | 'u8' | 'u16' | 'u32' | 'u64'
    //                   ;
    //
    #[regex(
        r"(?x)
            # RAW_INT_LIT
            (
                ((0b|0B)[01]([01_]*[01])?)                         # INT_BINARY_LIT
                |
                ((0o|0O)[0-7]([0-7_]*[0-7])?)                      # INT_OCTAL_LIT
                |
                ((0x|0X)[0-9A-Fa-f]([0-9A-Fa-f_]*[0-9A-Fa-f])?)    # INT_HEX_LIT
                |
                ([0-9]([0-9_]*[0-9])?)                             # INT_DEC_LIT
            )
            # optional INT_TYPE_SUFFIX
            ([iu](8|16|32|64))?
        ",
        callback = parse_int_lit
    )]
    IntLit(IntLit),

    //           FLOAT_LIT ::= FLOAT_WHOLE_PART , FLOAT_FRACTION_PART , FLOAT_EXPONENT_PART? , FLOAT_TYPE_SUFFIX?
    //                       | FLOAT_WHOLE_PART , FLOAT_EXPONENT_PART , FLOAT_TYPE_SUFFIX?
    //                       | FLOAT_WHOLE_PART , FLOAT_TYPE_SUFFIX
    //                       ;
    //    FLOAT_WHOLE_PART ::= [0-9] , ([0-9_]* , [0-9])? ;
    // FLOAT_FRACTION_PART ::= "." , [0-9] , ([0-9_]* , [0-9])? ;
    // FLOAT_EXPONENT_PART ::= [eE] , [+-] , [0-9] , ([0-9_]* , [0-9])? ;
    //
    //   FLOAT_TYPE_SUFFIX ::= 'f32' | 'f64' ;
    #[regex(
        r"(?x)
            (
                [0-9]([0-9_]*[0-9])?                # FLOAT_WHOLE_PART
                \.[0-9]([0-9_]*[0-9])?              # FLOAT_FRACTION_PART
                ([eE][+-]?[0-9]([0-9_+]*[0-9])?)?   # FLOAT_EXPONENT_PART?
                (f(32|64))?                         # optional FLOAT_TYPE_SUFFIX
            )
            |
            (
                [0-9]([0-9_]*[0-9])?                # FLOAT_WHOLE_PART
                ([eE][+-]?[0-9]([0-9_+]*[0-9])?)    # FLOAT_EXPONENT_PART
                (f(32|64))?                         # optional FLOAT_TYPE_SUFFIX
            )
            |
            (
                [0-9]([0-9_]*[0-9])?                # FLOAT_WHOLE_PART
                (f(32|64))                          # mandatory FLOAT_TYPE_SUFFIX
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

    #[test_case(                   "//",                "", false ; "empty comment")]
    #[test_case(                  "// ",               " ", false ; "empty comment, trailing whitespace")]
    #[test_case( "// foo bar baz\nblah",  " foo bar baz\n",  true ; "basic inline comment, more content afterwards")]
    #[test_case(     "// foo bar baz\n",  " foo bar baz\n", false ; "basic inline comment, no content afterwards")]
    #[test_case(       "// foo bar baz",    " foo bar baz", false ; "inline comment at EOF")]
    #[test_case("// foo bar baz \nblah", " foo bar baz \n",  true ; "basic inline comment, trailing whitespace, more content afterwards")]
    #[test_case(    "// foo bar baz \n", " foo bar baz \n", false ; "basic inline comment, trailing whitespace, no content afterwards")]
    #[test_case(      "// foo bar baz ",   " foo bar baz ", false ; "inline comment at EOF, trailing whitespace")]
    fn inline_comments_parse_as_expected(
        input: &str,
        expected_content: &str,
        expect_more_content: bool,
    ) {
        // Given
        let mut lexer = Token::lexer(input);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        if let Token::InlineComment(str) = token {
            assert_eq!(expected_content, str.as_ref())
        } else {
            panic!("expected InlineComment, got {:?}", token)
        }

        let next_token = lexer.next();
        if expect_more_content {
            assert!(
                next_token.is_some(),
                "expected more content after token but got EOF"
            );
        } else {
            assert!(
                next_token.is_none(),
                "found unexpected content after token: {:?}",
                next_token
            );
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
        if let Token::MultilineComment(str) = token {
            assert_eq!(expected_content, str.as_ref())
        } else {
            panic!("expected MultilineComment, got {:?}", token)
        }

        let next_token = lexer.next();
        assert!(
            next_token.is_none(),
            "found unexpected content after token: {:?}",
            next_token
        );
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

        let next_token = lexer.next();
        assert!(
            next_token.is_none(),
            "found unexpected content after token: {:?}",
            next_token
        );
    }

    // untyped, and common formatting scenarios
    #[test_case(                                 "0b0",                   IntLit::Untyped(0) ; "0: base 2 lowercase prefix, no type")]
    #[test_case(                                 "0b1",                   IntLit::Untyped(1) ; "1: base 2 lowercase prefix, no type")]
    #[test_case(                           "0b1100100",                 IntLit::Untyped(100) ; "100: base 2 lowercase prefix, no type")]
    #[test_case(                       "0b111110_1000",                IntLit::Untyped(1000) ; "1000: base 2 lowercase prefix, no type, underscores")]
    #[test_case(                    "0b11__1110__1000",                IntLit::Untyped(1000) ; "1000: base 2 lowercase prefix, no type, multiple underscores")]
    #[test_case(   "0b1111111111111111111111111111111",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 2 lowercase prefix, no type")]
    #[test_case(                                 "0B0",                   IntLit::Untyped(0) ; "0: base 2 uppercase prefix, no type")]
    #[test_case(                                 "0B1",                   IntLit::Untyped(1) ; "1: base 2 uppercase prefix, no type")]
    #[test_case(                           "0B1100100",                 IntLit::Untyped(100) ; "100: base 2 uppercase prefix, no type")]
    #[test_case(                       "0B111110_1000",                IntLit::Untyped(1000) ; "1000: base 2 uppercase prefix, no type, underscores")]
    #[test_case(                    "0B11__1110__1000",                IntLit::Untyped(1000) ; "1000: base 2 uppercase prefix, no type, multiple underscores")]
    #[test_case(   "0B1111111111111111111111111111111",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 2 uppercase prefix, no type")]
    #[test_case(                                 "0o0",                   IntLit::Untyped(0) ; "0: base 8 lowercase prefix, no type")]
    #[test_case(                                 "0o1",                   IntLit::Untyped(1) ; "1: base 8 lowercase prefix, no type")]
    #[test_case(                               "0o144",                 IntLit::Untyped(100) ; "100: base 8 lowercase prefix, no type")]
    #[test_case(                             "0o17_50",                IntLit::Untyped(1000) ; "1000: base 8 lowercase prefix, no type, underscores")]
    #[test_case(                           "0o1_75__0",                IntLit::Untyped(1000) ; "1000: base 8 lowercase prefix, no type, multiple underscores")]
    #[test_case(                       "0o17777777777",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 8 lowercase prefix, no type")]
    #[test_case(                                 "0O0",                   IntLit::Untyped(0) ; "0: base 8 uppercase prefix, no type")]
    #[test_case(                                 "0O1",                   IntLit::Untyped(1) ; "1: base 8 uppercase prefix, no type")]
    #[test_case(                               "0O144",                 IntLit::Untyped(100) ; "100: base 8 uppercase prefix, no type")]
    #[test_case(                             "0O17_50",                IntLit::Untyped(1000) ; "1000: base 8 uppercase prefix, no type, underscores")]
    #[test_case(                           "0O1_75__0",                IntLit::Untyped(1000) ; "1000: base 8 uppercase prefix, no type, multiple underscores")]
    #[test_case(                       "0O17777777777",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 8 uppercase prefix, no type")]
    #[test_case(                                   "0",                   IntLit::Untyped(0) ; "0: base 10, no type")]
    #[test_case(                                   "1",                   IntLit::Untyped(1) ; "1: base 10, no type")]
    #[test_case(                                 "100",                 IntLit::Untyped(100) ; "100: base 10, no type")]
    #[test_case(                               "1_000",                IntLit::Untyped(1000) ; "1000: base 10, no type, underscores")]
    #[test_case(                             "1__00_0",                IntLit::Untyped(1000) ; "1000: base 10, no type, multiple underscores")]
    #[test_case(                          "2147483647",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 10, no type")]
    #[test_case(                                 "0x0",                   IntLit::Untyped(0) ; "0: base 16 lowercase prefix, no type")]
    #[test_case(                                 "0x1",                   IntLit::Untyped(1) ; "1: base 16 lowercase prefix, no type")]
    #[test_case(                                "0x64",                 IntLit::Untyped(100) ; "100: base 16 lowercase prefix, no type")]
    #[test_case(                              "0x3_e8",                IntLit::Untyped(1000) ; "1000: base 16 lowercase prefix, no type, underscores")]
    #[test_case(                            "0x3__e_8",                IntLit::Untyped(1000) ; "1000: base 16 lowercase prefix, no type, multiple underscores")]
    #[test_case(                          "0x7fffffff",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 16 lowercase prefix, no type")]
    #[test_case(                          "0x7ffFFffF",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 16 lowercase prefix, mixed case, no type")]
    #[test_case(                                 "0X0",                   IntLit::Untyped(0) ; "0: base 16 uppercase prefix, no type")]
    #[test_case(                                 "0X1",                   IntLit::Untyped(1) ; "1: base 16 uppercase prefix, no type")]
    #[test_case(                                "0X64",                 IntLit::Untyped(100) ; "100: base 16 uppercase prefix, no type")]
    #[test_case(                              "0X3_E8",                IntLit::Untyped(1000) ; "1000: base 16 uppercase prefix, no type, underscores")]
    #[test_case(                            "0X3__E_8",                IntLit::Untyped(1000) ; "1000: base 16 uppercase prefix, no type, multiple underscores")]
    #[test_case(                          "0X7FFFFFFF",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 16 uppercase prefix, no type")]
    #[test_case(                          "0X7ffFFffF",          IntLit::Untyped(2147483647) ; "2147483647: max i32 value, base 16 uppercase prefix, mixed case, no type")]
    // i8
    #[test_case(                                 "0i8",                        IntLit::I8(0) ; "0: min i8 value, base 10, i8")]
    #[test_case(                                "52i8",                       IntLit::I8(52) ; "52: base 10, i8")]
    #[test_case(                               "127i8",                      IntLit::I8(127) ; "127: max i8 value, base 10, i8")]
    // i16
    #[test_case(                                "0i16",                       IntLit::I16(0) ; "0: min i16 value, base 10, i16")]
    #[test_case(                               "52i16",                      IntLit::I16(52) ; "52: base 10, i16")]
    #[test_case(                            "32767i16",                   IntLit::I16(32767) ; "32767: max i16 value, base 10, i16")]
    // i32
    #[test_case(                                "0i32",                       IntLit::I32(0) ; "0: min i32 value, base 10, i32")]
    #[test_case(                               "52i32",                      IntLit::I32(52) ; "52: base 10, i32")]
    #[test_case(                       "2147483647i32",              IntLit::I32(2147483647) ; "2147483647: max i32 value, base 10, i32")]
    // i64
    #[test_case(                                "0i64",                    IntLit::I64(0i64) ; "0: min i64 value, base 10, i64")]
    #[test_case(                               "52i64",                   IntLit::I64(52i64) ; "52: base 10, i64")]
    #[test_case(              "9223372036854775807i64",  IntLit::I64(9223372036854775807i64) ; "9223372036854775807: max i64 value, base 10, i64")]
    // u8
    #[test_case(                                 "0u8",                        IntLit::U8(0) ; "0: min u8 value, base 10, u8")]
    #[test_case(                                "52u8",                       IntLit::U8(52) ; "52: base 10, u8")]
    #[test_case(                               "255u8",                      IntLit::U8(255) ; "255: max u8 value, base 10, u8")]
    // u16
    #[test_case(                                "0u16",                       IntLit::U16(0) ; "0: min u16 value, base 10, u16")]
    #[test_case(                               "52u16",                      IntLit::U16(52) ; "52: base 10, u16")]
    #[test_case(                            "65535u16",                   IntLit::U16(65535) ; "65535: max u16 value, base 10, u16")]
    // u32
    #[test_case(                                "0u32",                       IntLit::U32(0) ; "0: min u32 value, base 10, u32")]
    #[test_case(                               "52u32",                      IntLit::U32(52) ; "52: base 10, u32")]
    #[test_case(                       "4294967295u32",              IntLit::U32(4294967295) ; "4294967295: max u32 value, base 10, u32")]
    // u64
    #[test_case(                                "0u64",                    IntLit::U64(0u64) ; "0: min u64 value, base 10, u64")]
    #[test_case(                               "52u64",                   IntLit::U64(52u64) ; "52: base 10, u64")]
    #[test_case(             "18446744073709551615u64", IntLit::U64(18446744073709551615u64) ; "18446744073709551615: max u64 value, base 10, u64")]
    fn int_literals_are_parsed_correctly(input: &str, expected: IntLit) {
        // Given
        let mut lexer = Token::lexer(input);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        if let Token::IntLit(int_lit) = token {
            assert_eq!(int_lit, expected);
        } else {
            panic!("expected IntLit, got {:?}", token);
        }

        let next_token = lexer.next();
        assert!(
            next_token.is_none(),
            "found unexpected content after token: {:?}",
            next_token
        );
    }

    // untyped
    #[test_case(                 "0.123",       FloatLit::Untyped(0.123) ; "0.123: decimal, no exponent, no underscores, untyped")]
    #[test_case(            "32_768.123",   FloatLit::Untyped(32768.123) ; "32768.123: decimal, no exponent, underscores, untyped")]
    #[test_case(         "32__768.12__3",   FloatLit::Untyped(32768.123) ; "32768.123: decimal, no exponent, multiple underscores, untyped")]
    #[test_case(                 "0e123",       FloatLit::Untyped(0e123) ; "0e123: no decimal, unsigned lowercase exponent, no underscores, untyped")]
    #[test_case(            "32_768e123",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, unsigned lowercase exponent, underscores, untyped")]
    #[test_case(         "32__768e1__23",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, unsigned lowercase exponent, multiple underscores, untyped")]
    #[test_case(                "0e+123",       FloatLit::Untyped(0e123) ; "0e123: no decimal, positive lowercase exponent, no underscores, untyped")]
    #[test_case(           "32_768e+123",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, positive lowercase exponent, underscores, untyped")]
    #[test_case(        "32__768e+1__23",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, positive lowercase exponent, multiple underscores, untyped")]
    #[test_case(                "0e-123",      FloatLit::Untyped(0e-123) ; "0e-123: no decimal, negative lowercase exponent, no underscores, untyped")]
    #[test_case(           "32_768e-123",  FloatLit::Untyped(32768e-123) ; "32768e-123: no decimal, negative lowercase exponent, underscores, untyped")]
    #[test_case(        "32__768e-12__3",  FloatLit::Untyped(32768e-123) ; "32768e-123: no decimal, negative lowercase exponent, multiple underscores, untyped")]
    #[test_case(                 "0E123",       FloatLit::Untyped(0e123) ; "0e123: no decimal, unsigned uppercase exponent, no underscores, untyped")]
    #[test_case(            "32_768E123",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, unsigned uppercase exponent, underscores, untyped")]
    #[test_case(         "32__768E1__23",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, unsigned uppercase exponent, multiple underscores, untyped")]
    #[test_case(                "0E+123",       FloatLit::Untyped(0e123) ; "0e123: no decimal, positive uppercase exponent, no underscores, untyped")]
    #[test_case(           "32_768E+123",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, positive uppercase exponent, underscores, untyped")]
    #[test_case(        "32__768E+1__23",   FloatLit::Untyped(32768e123) ; "32768e123: no decimal, positive uppercase exponent, multiple underscores, untyped")]
    #[test_case(                "0E-123",      FloatLit::Untyped(0e-123) ; "0e-123: no decimal, negative uppercase exponent, no underscores, untyped")]
    #[test_case(           "32_768E-123",  FloatLit::Untyped(32768e-123) ; "32768e-123: no decimal, negative uppercase exponent, underscores, untyped")]
    #[test_case(        "32__768E-12__3",  FloatLit::Untyped(32768e-123) ; "32768e-123: no decimal, negative uppercase exponent, multiple underscores, untyped")]
    // f32
    #[test_case(                  "0f32",            FloatLit::F32(0f32) ; "0: no decimal, no exponent, no underscores, f32 suffix")]
    #[test_case(             "32_768f32",        FloatLit::F32(32768f32) ; "32768: no decimal, no exponent, underscores, f32 suffix")]
    #[test_case(            "32__768f32",        FloatLit::F32(32768f32) ; "32768: no decimal, no exponent, multiple underscores, f32 suffix")]
    #[test_case(      "32__768.12__3f32",    FloatLit::F32(32768.123f32) ; "32768.123: decimal, no exponent, multiple underscores, f32 suffix")]
    #[test_case(      "32__768e1__2f32",      FloatLit::F32(32768e12f32) ; "32768e12: no decimal, unsigned lowercase exponent, multiple underscores, f32 suffix")]
    #[test_case(     "32__768e+1__2f32",      FloatLit::F32(32768e12f32) ; "32768e12: no decimal, positive lowercase exponent, multiple underscores, f32 suffix")]
    #[test_case(     "32__768E-1__2f32",     FloatLit::F32(32768E-12f32) ; "32768e-12: no decimal, negative uppercase exponent, multiple underscores, f32 suffix")]
    // f64
    #[test_case(                 "0f64",             FloatLit::F64(0f64) ; "0: no decimal, no exponent, no underscores, f64 suffix")]
    #[test_case(            "32_768f64",         FloatLit::F64(32768f64) ; "32768: no decimal, no exponent, underscores, f64 suffix")]
    #[test_case(           "32__768f64",         FloatLit::F64(32768f64) ; "32768: no decimal, no exponent, multiple underscores, f64 suffix")]
    #[test_case(   "32__768.12__234f64",   FloatLit::F64(32768.12234f64) ; "32768.12234: decimal, no exponent, multiple underscores, f64 suffix")]
    #[test_case(     "32__768e1__23f64",     FloatLit::F64(32768e123f64) ; "32768e123: no decimal, unsigned lowercase exponent, multiple underscores, f64 suffix")]
    #[test_case(    "32__768e+1__23f64",     FloatLit::F64(32768e123f64) ; "32768e123: no decimal, positive lowercase exponent, multiple underscores, f64 suffix")]
    #[test_case(    "32__768E-1__23f64",    FloatLit::F64(32768E-123f64) ; "32768e-123: no decimal, negative uppercase exponent, multiple underscores, f64 suffix")]
    fn float_literals_are_parsed_correctly(input: &str, expected: FloatLit) {
        // Given
        let mut lexer = Token::lexer(input);

        // When
        let token = lexer.next().unwrap().unwrap();

        // Then
        if let Token::FloatLit(float_lit) = token {
            assert_eq!(float_lit, expected);
        } else {
            panic!("expected FloatLit, got {:?}", token);
        }

        let next_token = lexer.next();
        assert!(
            next_token.is_none(),
            "found unexpected content after token: {:?}",
            next_token
        );
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

        let next_token = lexer.next();
        assert!(
            next_token.is_none(),
            "found unexpected content after token: {:?}",
            next_token
        );
    }
}
