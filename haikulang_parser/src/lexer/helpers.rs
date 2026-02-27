use crate::error::ParserError;
use crate::lexer::literals::{FloatLit, IntLit, StrLit};
use crate::lexer::token::Token;
use std::str::FromStr;

type HelperResult<T> = Result<T, ParserError>;

pub fn parse_unknown_input(lex: &mut logos::Lexer<Token>) -> ParserError {
    let text = &lex.slice()[lex.span()];
    ParserError::UnknownToken(text.to_string())
}

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

pub fn parse_string(lex: &mut logos::Lexer<Token>) -> HelperResult<StrLit> {
    let unparsed = lex.slice();

    if !unparsed.ends_with('"') {
        return Err(ParserError::UnclosedStringLit(unparsed.to_string()));
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
                    return Err(ParserError::InvalidStringLit(
                        "unexpected line feed encountered".to_string(),
                    ));
                }
                '\r' => {
                    return Err(ParserError::InvalidStringLit(
                        "unexpected carriage return encountered".to_string(),
                    ));
                }
                c if c.is_control() => {
                    return Err(ParserError::InvalidStringLit(format!(
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

fn parse_string_escape_char(text: &str, offset: usize) -> HelperResult<(&str, usize)> {
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
                Err(err) => Err(ParserError::InvalidStringLit(format!(
                    "failed to parse invalid unicode codepoint \\u{}: {}",
                    codepoint_string, err
                ))),
            }
        }
        other => Err(ParserError::InvalidStringLit(format!(
            "unknown escape sequence in string: \\{}",
            other.escape_default()
        ))),
    }
}

pub fn parse_int_lit(lex: &mut logos::Lexer<Token>) -> HelperResult<IntLit> {
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

fn parse_int_lit_radix(text: &str, radix: u32) -> HelperResult<IntLit> {
    // Remove underscores that are only used for visual clarity.
    let sanitised_text = text.replace("_", "");

    // Split on any suffix if present.
    let (number, suffix) = sanitised_text.split_at(
        sanitised_text
            .find(|c| matches!(c, 'i' | 'u'))
            .unwrap_or_else(|| sanitised_text.len()),
    );

    // Map to the expected type.
    let result = match suffix {
        "i8" => i8::from_str_radix(number, radix).map(IntLit::I8),
        "i16" => i16::from_str_radix(number, radix).map(IntLit::I16),
        "i32" => i32::from_str_radix(number, radix).map(IntLit::I32),
        "i64" => i64::from_str_radix(number, radix).map(IntLit::I64),
        "u8" => u8::from_str_radix(number, radix).map(IntLit::U8),
        "u16" => u16::from_str_radix(number, radix).map(IntLit::U16),
        "u32" => u32::from_str_radix(number, radix).map(IntLit::U32),
        "u64" => u64::from_str_radix(number, radix).map(IntLit::U64),
        // If we have no suffix, we treat it as an i32 for now.
        _ => i32::from_str_radix(number, radix).map(IntLit::Untyped),
    };

    result.map_err(|err| {
        ParserError::InvalidIntLit(format!(
            "failed to parse base-{} {} value {:?}: {}",
            radix,
            if suffix.is_empty() { "int" } else { suffix },
            text,
            err
        ))
    })
}

pub fn parse_float_lit(lex: &mut logos::Lexer<Token>) -> HelperResult<FloatLit> {
    // Remove underscores that are only used for visual clarity.
    let sanitised_text = lex.slice().replace("_", "");

    // Split on any suffix if present.
    let (number, suffix) = sanitised_text.split_at(
        sanitised_text
            .find(|c| matches!(c, 'f'))
            .unwrap_or_else(|| sanitised_text.len()),
    );

    // Map to the expected type.
    let result = match suffix {
        "f32" => f32::from_str(number).map(FloatLit::F32),
        "f64" => f64::from_str(number).map(FloatLit::F64),
        // If we have no suffix, we treat it as a f64 for now.
        _ => f64::from_str(number).map(FloatLit::Untyped),
    };

    result.map_err(|err| {
        ParserError::InvalidFloatLit(format!(
            "failed to parse base-10 {} value {:?}: {}",
            if suffix.is_empty() { "float" } else { suffix },
            sanitised_text,
            err
        ))
    })
}
