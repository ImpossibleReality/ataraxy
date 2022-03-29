use az::SaturatingCast;

pub trait Integer {
    const MIN: f64;
    const MAX: f64;
    fn from_i64(num: i64) -> Self;
}

impl Integer for u8 {
    const MIN: f64 = u8::MIN as f64;
    const MAX: f64 = u8::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as u8
    }
}

impl Integer for u16 {
    const MIN: f64 = u16::MIN as f64;
    const MAX: f64 = u16::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as u16
    }
}

impl Integer for u32 {
    const MIN: f64 = u32::MIN as f64;
    const MAX: f64 = u32::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as u32
    }
}

impl Integer for u64 {
    const MIN: f64 = u64::MIN as f64;
    const MAX: f64 = u64::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as u64
    }
}

impl Integer for u128 {
    const MIN: f64 = u128::MIN as f64;
    const MAX: f64 = u128::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as u128
    }
}

impl Integer for i8 {
    const MIN: f64 = i8::MIN as f64;
    const MAX: f64 = i8::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as i8
    }
}

impl Integer for i16 {
    const MIN: f64 = i16::MIN as f64;
    const MAX: f64 = i16::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as i16
    }
}

impl Integer for i32 {
    const MIN: f64 = i32::MIN as f64;
    const MAX: f64 = i32::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as i32
    }
}

impl Integer for i64 {
    const MIN: f64 = i64::MIN as f64;
    const MAX: f64 = i64::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num
    }
}
impl Integer for i128 {
    const MIN: f64 = i128::MIN as f64;
    const MAX: f64 = i128::MAX as f64;
    fn from_i64(num: i64) -> Self {
        num as i128
    }
}

pub trait Float {
    const MIN: f64;
    const MAX: f64;
    fn from_f64(num: f64) -> Self;
}

impl Float for f32 {
    const MIN: f64 = f32::MIN as f64;
    const MAX: f64 = f32::MAX as f64;
    fn from_f64(num: f64) -> Self {
        num as f32
    }
}

impl Float for f64 {
    const MIN: f64 = f64::MIN as f64;
    const MAX: f64 = f64::MAX as f64;
    fn from_f64(num: f64) -> Self {
        num as f64
    }
}
