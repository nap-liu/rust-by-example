//! # 表达式

///  大多数情况下 Rust 程序都是由一系列语句组成的
fn main() {
    // 语句
    // 语句
    // 语句
}

/// Rust 中最常用的两种是 变量声明和表达式（表达式使用`;`结尾）
fn main() {
    // 声明变量绑定到表达式结果
    let x = 5;

    // 单一的表达式;
    x;
    x + 1;
    15;
}

/// 多行表达式也是一样的，多行表达式返回的是最后一行的表达式结果，
/// 如果最后一行表达式使用了`;`号结尾的话，则该表达式块会返回 `()`
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
