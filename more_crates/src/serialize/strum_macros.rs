use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr};

/// Display 实现了to_string
/// EnumString 实现了属性装饰宏
/// 为其设置 serde 的 alias 属性可以通过 serde 的 form_str
#[derive(Debug, Display, EnumString, Eq, PartialEq, Deserialize, Serialize)]
enum SpeedUnit {
    #[serde(alias = "km/h")]
    #[strum(serialize = "km/h")]
    Kmph,
    #[serde(alias = "m/s")]
    #[strum(serialize = "m/s")]
    Mps,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
struct Car {
    name: String,
    speed: u8,
    unit: SpeedUnit,
}

// 定义一个带有整数值的枚举
#[derive(FromRepr, Display, Debug, Eq, PartialEq, Deserialize, Serialize)]
enum SexType {
    #[strum(serialize = "1")]
    Male = 1,
    #[strum(serialize = "2")]
    Female = 2,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
struct User {
    name: String,
    sex: SexType,
}

#[cfg(test)]
mod test_strum_macros {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_speed_unit_from() {
        let unit = SpeedUnit::from_str("km/h").unwrap();
        assert_eq!(unit, SpeedUnit::Kmph);
    }

    #[test]
    #[should_panic]
    fn test_speed_unit_from_fail() {
        SpeedUnit::from_str("kmh").unwrap();
    }

    /// 如果不为其添加 serde 的 alias 属性 则会报错
    #[test]
    fn test_speed_unit_json_fail() {
        let json_str = r#""km/h""#;
        let t: SpeedUnit = serde_json::from_str(json_str).unwrap();
        // 并没有成功将其转换
        // value: Error("unknown variant `km/h`, expected `Kmph` or `Mps`", line: 1, column: 6)
        assert_eq!(t, SpeedUnit::Kmph)
    }

    #[test]
    fn test_speed_unit_from_struct() {
        let json_str = r#"{"name":"Coca Cola","speed":120,"unit":"km/h"}"#;
        let car = serde_json::from_str::<Car>(json_str).unwrap();

        assert_eq!(
            car,
            Car {
                name: "Coca Cola".to_string(),
                speed: 120,
                unit: SpeedUnit::Kmph,
            }
        );
    }

    #[test]
    fn test_speed_unit_to_string() {
        let kmph = SpeedUnit::Kmph;
        assert_eq!(kmph.to_string(), "km/h");

        let mps = SpeedUnit::Mps;
        assert_eq!(mps.to_string(), "m/s")
    }

    #[test]
    fn test_sex_type_from_repr() {
        let num = 1;
        let t = SexType::from_repr(num).unwrap();
        assert_eq!(t, SexType::Male)
    }

    #[test]
    #[should_panic]
    fn test_sex_type_from_fail() {
        let json_str = "1";
        // 无法从字符串中转为该枚举
        let t = serde_json::from_str::<SexType>(json_str).unwrap();
        assert_eq!(t, SexType::Male)
    }

    #[test]
    #[should_panic]
    fn test_sex_type_to_struct_fail() {
        let json_str = r#"{"name":"Tom","sex":1}"#;
        // 在json字符串中也无法转换为该枚举
        let user = serde_json::from_str::<User>(json_str).unwrap();

        assert_eq!(
            user,
            User {
                name: "Tom".to_string(),
                sex: SexType::Male
            }
        )
    }
}
