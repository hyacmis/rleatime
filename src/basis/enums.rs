#![allow(dead_code)]

#[derive(PartialEq, Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
    V8(String),
}

impl IpAddress {
    fn cases(&self) -> String {
        match self {
            IpAddress::V4(_, _, _, _) => "IPv4".to_string(),
            IpAddress::V6(_) => "IPv6".to_string(), // 对于不需要处理的选项
            _ => "NONE".to_string(),
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        // 在这里定义方法体
        println!("example")
    }
}

#[cfg(test)]
mod test_enum {
    use super::*;

    #[test]
    fn test_ip_address() {
        let ipv4 = IpAddress::V4(127, 0, 0, 1);
        assert_eq!(ipv4, IpAddress::V4(127, 0, 0, 1));

        let ipv6 = IpAddress::V6(String::from("fe80::7436:aba9:c779:b39d%12"));

        assert_eq!(
            ipv6,
            IpAddress::V6(String::from("fe80::7436:aba9:c779:b39d%12"))
        );
    }

    #[test]
    fn test_ip_cases() {
        let ipv4 = IpAddress::V4(127, 0, 0, 1);
        assert_eq!(ipv4.cases(), "IPv4".to_string());

        let ipv6 = IpAddress::V6(String::from("fe80::7436:aba9:c779:b39d%12"));
        assert_eq!(ipv6.cases(), "IPv6".to_string());

        let ipv6 = IpAddress::V8(String::from("V8"));
        assert_eq!(ipv6.cases(), "NONE".to_string());
    }
}
