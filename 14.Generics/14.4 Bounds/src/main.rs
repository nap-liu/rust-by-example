//!
//! 泛型约束（Bounds）
//!
//! 再使用泛型的时候，很多时候都会需要要求泛型必须拥有哪些特性，如果不能满足的话不能使用该函数，
//! 所有就有了泛型的约束，要求传递的参数必须实现某些特定的特性。
//!

fn example01() {
    use std::fmt::Display;

    // 该函数定义了一个泛型类型 `T`，并且约束该类型必须实现要先实现 `Display` 特性。
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    struct S<T: Display>(T);

    // 错误! `Vec<T>` 没有实现 `Display` 特性
    // let s = S(vec![1]);
    // 移除注释查看错误
}

// 导入格式化标签 `{:?}` 必须要实现的特性。
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// 泛型 `T` 必须实现 `Debug` 特性
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// 泛型 `T` 必须实现 `HasArea` 特性，只有实现了这个特性才能使用 `HasArea` 上的 `area` 方法。
fn area<T>(t: &T) -> f64
where
    // 另外一种泛型约束的语法，这样可以保持函数签名是干净的
    T: HasArea,
{
    t.area()
}

/// 即使是空的特性也可以用作约束，就像是标准库提供的 `Copy`、`Eq` 一样
fn testcase_empty_bounds() {
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // 这个函数只允许实现了 `Red` 特性的参数调用。
    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }

    // 这个函数只允许实现了 `Blue` 特性的参数调用。
    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` 不能传递 `blue_jay`，因为函数进行了约束，反过来也一样。
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    // println!("A turkey is {}", red(&_turkey));
    // ^ TODO: 移除注释查看错误
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // ^ TODO: 移除注释查看错误
    // | Error: 这两个都没有实现相关的 `Debug` 或 `HasArea` 特性。
}
