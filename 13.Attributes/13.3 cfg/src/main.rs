//! `cfg` 属性和宏
//!
//! `cfg` 是一个条件属性，当条件成立的时候才会生效，常用的两种形式如下。
//!
//! - `cfg` 当做属性来使用：`#[cfg(...)]` 指定的位置
//! - `cfg` 当做条件计算来用：`cfg!(...)` 条件表达式
//!
//! 当做属性来使用的时候是在编译阶段执行的，当做条件表达式的时候是在运行时执行的。
//! 两种形式都接受相同的参数类型。
//!
//! `cfg!` 宏不像是 `#[cfg]`在编译阶段直接移除掉了条件为 `false` 的代码，
//! 而是保留了所有的代码在运行时执行判断。
//!

// 这个函数只会在 linux 系统下才会被编译到可执行文件中
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// 这个函数会在不是 linux 系统的任意系统下都会编译到可执行文件中
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");

    // 不管编译的系统是什么，条件分支的代码都会被编译到可执行文件中
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
