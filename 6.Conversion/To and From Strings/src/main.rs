use std::{fmt, str::FromStr};

struct Circle {
    radius: i32,
}

/// 很多时候需要把数据结构转化成字符串，Rust提供了 ToString 特性，
/// 只需要实现 ToString 特性就可以把结构转换成 String类型，
/// 但是这个方法只能给结构体提供一个 to_string() 方法，
/// 而且不能自动支持类似 println!() 的使用，
/// 通常我们实现的是 fmt::Display 特性，实现了该特性，会自动实现 ToString 特性
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}

/// 实现 FromStr 的话，就可以通过 str 来构造对应的类型了
impl FromStr for Circle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(v) => Ok(Circle { radius: v }),
            Err(_) => Err(()),
        }
    }
}

fn main() {
    let circle = Circle { radius: 6 };

    // 通过 parse 方法传递泛型来通过字符串构造我们的自定义类型
    let circle2 = "6".parse::<Circle>().unwrap();

    // 使用 通过实现 fmt::Dispaly 特性提供的 to_string() 方法来获得字符串
    println!("{}", circle.to_string());

    assert_eq!(circle.radius, circle2.radius)
}
