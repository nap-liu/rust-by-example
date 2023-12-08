use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl Display for City {
    // f 是一个缓冲区（Buffer），这个函数把格式化好的内容写到这个缓冲区中。
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 和 `format!` 这两个宏非常相似,
        // 只不过 `write!` 宏是向指定的缓冲区（第一个参数）中写数据，而`format!`是返回一个格式化好的字符串。
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 实现 {} 特性 输出 RGB (x, x, x) 格式
impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)
    }
}

// 实现 {:x} 特性 输出 0xaabbcc 格式
impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 使用 {:x} 格式把数字转成小写16进制文本
        let lower_hex = |x: u8| format!("{:x}", x);
        // 合并输出三个独立颜色单元的文字并自动左侧补齐空格
        write!(
            f,
            "0x{:0>2}{:0>2}{:0>2}",
            lower_hex(self.red),
            lower_hex(self.green),
            lower_hex(self.blue),
        )
    }
}

// 实现 {:X} 特性 输出 0xAABBCC 格式
impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 使用 {:X} 格式把数字转成大写16进制文本
        let upper_hex = |x: u8| format!("{:X}", x);
        // 同上
        write!(
            f,
            "0x{:0>2}{:0>2}{:0>2}",
            upper_hex(self.red),
            upper_hex(self.green),
            upper_hex(self.blue),
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // 三种格式输出 `fmt::Display` `fmt::UpperHex` `fmt::LowerHex`
        // 字符串，大写十六进制，小写十六进制
        println!("{0:?} {0:x} {0:X}", color);
    }
}
