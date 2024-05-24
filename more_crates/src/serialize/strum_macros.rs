use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr};

/// Display 实现了to_string
/// EnumString 实现了属性装饰宏
/// 为其设置 serde 的 alias 属性可以通过 serde 的 form_str
#[derive(Debug, Display, EnumString, PartialEq, Deserialize, Serialize)]
enum SpeedUnit {
    // #[serde(alias = "km/h")]
    #[strum(serialize = "km/h")]
    Kmph,
    // #[serde(alias = "m/s")]
    #[strum(serialize = "m/s")]
    Mps,
}

// 定义一个带有整数值的枚举
#[derive(FromRepr, Display, Debug, PartialEq)]
#[allow(dead_code)]
enum SexType {
    Male = 1,
    Female = 2,
}

#[cfg(test)]
mod test_strum_macros {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from() {
        let unit = SpeedUnit::from_str("km/h").unwrap();
        assert_eq!(unit, SpeedUnit::Kmph);
    }

    #[test]
    #[should_panic]
    fn test_from_fail() {
        SpeedUnit::from_str("kmh").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_serde_json_fail() {
        let json_str = r#""km/h""#;
        let t: SpeedUnit = serde_json::from_str(json_str).unwrap();
        // 并没有成功将其转换
        // value: Error("unknown variant `km/h`, expected `Kmph` or `Mps`", line: 1, column: 6)
        assert_eq!(t, SpeedUnit::Kmph)
    }

    #[test]
    fn test_to_string() {
        let kmph = SpeedUnit::Kmph;
        assert_eq!(kmph.to_string(), "km/h");

        let mps = SpeedUnit::Mps;
        assert_eq!(mps.to_string(), "m/s")
    }

    #[test]
    fn test_from_repr() {
        let num = 1;
        let t = SexType::from_repr(num).unwrap();
        assert_eq!(t, SexType::Male)
    }
}
