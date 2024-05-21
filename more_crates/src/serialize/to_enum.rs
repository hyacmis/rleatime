use strum_macros::{Display, EnumString, FromRepr};

#[derive(Debug, Display, EnumString, PartialEq)]
enum SpeedUnit {
    #[strum(serialize = "km/h")]
    Kmph,
    #[strum(serialize = "m/s")]
    Mps,
}

#[cfg(test)]
mod test_enum_serialize {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_speed_unit_parse() {
        let unit = SpeedUnit::from_str("km/h").unwrap();
        assert_eq!(unit, SpeedUnit::Kmph);
    }

    #[test]
    #[should_panic]
    fn test_speed_unit_parse_fail() {
        SpeedUnit::from_str("kmh").unwrap();
    }

    #[test]
    fn test_speed_unit_unparse() {
        let value = SpeedUnit::Kmph.to_string();
        assert_eq!(value, "km/h")
    }
}

// 定义一个带有整数值的枚举
#[derive(FromRepr, Debug, PartialEq)]
#[allow(dead_code)]
enum SexType {
    Male = 1,
    Female = 2,
}

#[cfg(test)]
mod num_test {
    use super::*;

    #[test]
    fn test_num_to_enum() {
        let num = 1;
        let t = SexType::from_repr(num).unwrap();
        assert_eq!(t, SexType::Male)
    }
}
