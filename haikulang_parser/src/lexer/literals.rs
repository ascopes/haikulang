#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntLit {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    Untyped(i32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FloatLit {
    F32(f32),
    F64(f64),

    Untyped(f64),
}

pub type StrLit = Box<str>;
