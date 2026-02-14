use crate::lexer::error::LexerError;
use crate::lexer::token::{FloatLit, IntLit, StrLit, Token};
use std::str::FromStr;

pub fn parse_inline_comment(lex: &mut logos::Lexer<Token>) -> StrLit {
    let text = lex.slice();
    // Remove the leading "//"
    text[2..].to_string().into_boxed_str()
}

pub fn parse_multiline_comment(lex: &mut logos::Lexer<Token>) -> StrLit {
    let text = lex.slice();
    // Remove the leading "/*" and trailing "*/"
    text[2..text.len() - 2].to_string().into_boxed_str()
}

pub fn parse_identifier(lex: &mut logos::Lexer<Token>) -> StrLit {
    Box::from(lex.slice())
}

pub fn parse_string(lex: &mut logos::Lexer<Token>) -> Result<StrLit, LexerError> {
    let unparsed = lex.slice();

    if !unparsed.ends_with('"') {
        return Err(LexerError::UnclosedStringLit(unparsed.to_string()));
    }

    let mut parsed = String::new();

    // Start at 1, end at len-1 to remove open and close quotes.
    let mut offset = 1usize;

    while offset < unparsed.len() - 1 {
        match &unparsed[offset..offset + 1] {
            "\\" => {
                offset += 1;
                let (parsed_escape, length) = parse_string_escape_char(unparsed, offset)?;
                parsed.push_str(parsed_escape);
                offset += length;
            }
            c => match c.chars().nth(0).unwrap() {
                '\n' => {
                    return Err(LexerError::InvalidStringLit(
                        "unexpected line feed encountered".to_string(),
                    ));
                }
                '\r' => {
                    return Err(LexerError::InvalidStringLit(
                        "unexpected carriage return encountered".to_string(),
                    ));
                }
                c if c.is_control() => {
                    return Err(LexerError::InvalidStringLit(format!(
                        "unexpected control byte sequence encountered: {}",
                        c.escape_unicode(),
                    )));
                }
                _ => {
                    offset += 1;
                    parsed.push_str(c);
                }
            },
        }
    }

    Ok(parsed.into_boxed_str())
}

fn parse_string_escape_char(text: &str, offset: usize) -> Result<(&str, usize), LexerError> {
    // In theory this could be malformed, but we validate the format for the most part in the regex
    // before getting this far, so we can make assumptions here.
    match &text[offset..offset + 1] {
        "\\" => Ok(("\\", 1)),
        "\"" => Ok(("\"", 1)),
        "n" => Ok(("\n", 1)),
        "r" => Ok(("\r", 1)),
        "t" => Ok(("\t", 1)),
        "u" => {
            let codepoint_string = &text[offset + 1..offset + 5];
            let bytes = codepoint_string.as_bytes();
            match str::from_utf8(bytes) {
                Ok(utf8_char) => Ok((utf8_char, 5)),
                Err(err) => Err(LexerError::InvalidStringLit(format!(
                    "failed to parse invalid unicode codepoint \\u{}: {}",
                    codepoint_string, err
                ))),
            }
        }
        other => Err(LexerError::InvalidStringLit(format!(
            "unknown escape sequence in string: \\{}",
            other.escape_default()
        ))),
    }
}

pub fn parse_int_lit(lex: &mut logos::Lexer<Token>) -> Result<IntLit, LexerError> {
    let text = lex.slice();

    if text.starts_with("0b") || text.starts_with("0B") {
        parse_int_lit_radix(&text[2..text.len()], 2)
    } else if text.starts_with("0o") || text.starts_with("0O") {
        parse_int_lit_radix(&text[2..text.len()], 8)
    } else if text.starts_with("0x") || text.starts_with("0X") {
        parse_int_lit_radix(&text[2..text.len()], 16)
    } else {
        parse_int_lit_radix(&text, 10)
    }
}

fn parse_int_lit_radix(text: &str, radix: u32) -> Result<IntLit, LexerError> {
    // Remove any underscores, we do not keep them in the final value.
    // In theory this could be malformed, but we validate the format in the regex before
    // getting this far.
    IntLit::from_str_radix(text.replace("_", "").as_str(), radix).map_err(|err| {
        LexerError::InvalidIntLit(format!(
            "failed to parse base-{} value {:?}: {}",
            radix, text, err
        ))
    })
}

pub fn parse_float_lit(lex: &mut logos::Lexer<Token>) -> Result<FloatLit, LexerError> {
    // Remove any underscores, we do not keep them in the final value.
    // In theory this could be malformed, but we validate the format in the regex before
    // getting this far.
    let text = lex.slice();
    FloatLit::from_str(text.replace("_", "").as_str()).map_err(|err| {
        LexerError::InvalidFloatLit(format!(
            "failed to parse base-10 float value {:?}: {}",
            text, err
        ))
    })
}
