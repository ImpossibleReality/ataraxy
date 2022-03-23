pub trait Integer {
    const MIN: i128;
    const MAX: u128;
    fn from_i64(num: i64) -> Self;
}

impl Integer for u8 {
    const MIN: i128 = u8::MIN as i128;
    const MAX: u128 = u8::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as u8
    }
}

impl Integer for u16 {
    const MIN: i128 = u16::MIN as i128;
    const MAX: u128 = u16::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as u16
    }
}

impl Integer for u32 {
    const MIN: i128 = u32::MIN as i128;
    const MAX: u128 = u32::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as u32
    }
}

impl Integer for u64 {
    const MIN: i128 = u64::MIN as i128;
    const MAX: u128 = u64::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as u64
    }
}

impl Integer for u128 {
    const MIN: i128 = u128::MIN as i128;
    const MAX: u128 = u128::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as u128
    }
}

impl Integer for i8 {
    const MIN: i128 = i8::MIN as i128;
    const MAX: u128 = i8::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as i8
    }
}

impl Integer for i16 {
    const MIN: i128 = i16::MIN as i128;
    const MAX: u128 = i16::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as i16
    }
}

impl Integer for i32 {
    const MIN: i128 = i32::MIN as i128;
    const MAX: u128 = i32::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as i32
    }
}

impl Integer for i64 {
    const MIN: i128 = i64::MIN as i128;
    const MAX: u128 = i64::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num
    }
}
impl Integer for i128 {
    const MIN: i128 = i128::MIN as i128;
    const MAX: u128 = i128::MAX as u128;
    fn from_i64(num: i64) -> Self {
        num as i128
    }
}
