// 禁用未使用的代码警告
#![allow(dead_code)]

// 当不指明任意类型的时候，枚举项默认从 0 开始
enum Number {
    Zero,
    One,
    Two,
}

// 明确指明枚举项的值
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // 枚举可以强制转换成数字
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
