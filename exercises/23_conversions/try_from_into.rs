#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

// 实现从元组 (i16, i16, i16) 到 Color 的转换
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // 检查每个分量是否在 0..=255 范围内，并转换为 u8
        let red = tuple.0.try_into().map_err(|_| IntoColorError::IntConversion)?;
        let green = tuple.1.try_into().map_err(|_| IntoColorError::IntConversion)?;
        let blue = tuple.2.try_into().map_err(|_| IntoColorError::IntConversion)?;

        Ok(Color { red, green, blue })
    }
}

// 实现从数组 [i16; 3] 到 Color 的转换
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // 利用元组的 TryFrom 实现，简化代码
        (arr[0], arr[1], arr[2]).try_into()
    }
}

// 实现从切片 &[i16] 到 Color 的转换
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // 首先检查切片长度是否为 3
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        // 利用数组的 TryFrom 实现，简化代码
        [slice[0], slice[1], slice[2]].try_into()
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");

    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntoColorError::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }
}
