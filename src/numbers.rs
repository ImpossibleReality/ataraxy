pub enum NumberType {
    Integer,
    Float,
}

pub trait Number {
    const MIN: f64;
    const MAX: f64;
    fn from_f64(value: f64) -> Self;
    fn number_type() -> NumberType;
}

impl Number for u8 {
    const MIN: f64 = u8::MIN as f64;
    const MAX: f64 = u8::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as u8
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for u16 {
    const MIN: f64 = u16::MIN as f64;
    const MAX: f64 = u16::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as u16
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for u32 {
    const MIN: f64 = u32::MIN as f64;
    const MAX: f64 = u32::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as u32
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for u64 {
    const MIN: f64 = u64::MIN as f64;
    const MAX: f64 = u64::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as u64
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for i8 {
    const MIN: f64 = i8::MIN as f64;
    const MAX: f64 = i8::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as i8
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for i16 {
    const MIN: f64 = i16::MIN as f64;
    const MAX: f64 = i16::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as i16
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for i32 {
    const MIN: f64 = i32::MIN as f64;
    const MAX: f64 = i32::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as i32
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for i64 {
    const MIN: f64 = i64::MIN as f64;
    const MAX: f64 = i64::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as i64
    }
    fn number_type() -> NumberType {
        NumberType::Integer
    }
}

impl Number for f32 {
    const MIN: f64 = f32::MIN as f64;
    const MAX: f64 = f32::MAX as f64;
    fn from_f64(value: f64) -> Self {
        value as f32
    }
    fn number_type() -> NumberType {
        NumberType::Float
    }
}

impl Number for f64 {
    const MIN: f64 = f64::MIN;
    const MAX: f64 = f64::MAX;
    fn from_f64(value: f64) -> Self {
        value
    }
    fn number_type() -> NumberType {
        NumberType::Float
    }
}
