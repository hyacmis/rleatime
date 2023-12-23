#![allow(dead_code)]

#[derive(PartialEq, Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddress {
    fn cases(&self) {
        match self {
            IpAddress::V4(_, _, _, _) => {
                println!("IPv4");
            }
            IpAddress::V6(_) => {
                println!("IPv6");
            } // 对于不需要处理的选项
              // _ => {
              //     println!("default none",);
              // }
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

#[test]
fn test_ip_address() {
    let ipv4 = IpAddress::V4(127, 0, 0, 1);
    assert_eq!(ipv4, IpAddress::V4(127, 0, 0, 1));

    let ipv6 = IpAddress::V6(String::from("fe80::7436:aba9:c779:b39d%12"));
    assert_eq!(
        ipv6,
        IpAddress::V6(String::from("fe80::7436:aba9:c779:b39d%12"))
    );

    ipv4.cases();
    ipv6.cases();
}
