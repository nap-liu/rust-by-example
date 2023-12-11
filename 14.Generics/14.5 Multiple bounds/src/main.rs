//!
//! 多重泛型约束
//! 多重约束可以在每个约束之间使用 `+` 号连接，
//! 多个泛型之间使用 `,` 号间隔
//!
//! 同样支持在 `<T: bound + bound2>` 或者 where `T: bound + bound2` 中使用
//!
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array);
    // TODO ^ 移除注释查看错误

    compare_types(&array, &vec);
}
