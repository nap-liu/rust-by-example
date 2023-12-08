//!
//! 变量的定义和初始化通常是同时进行的。
//! Rust 允许把两个操作分开，先定义，后初始化
//! 使用变量的前提条件是变量必须被初始化了以后才能使用，否则在编译阶段就会报错
//!
//! 因为使用未初始化的变量通常会导致无法预知的行为，所以不允许使用未初始化的变量
//!
fn main() {
    // 声明一个变量 但不对其进行初始化
    let a_binding;

    {
        let x = 2;

        // 初始化变量值
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错! 使用了未初始化的变量
    println!("another binding: {}", another_binding);
    // FIXME ^ 注释这行代码

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
