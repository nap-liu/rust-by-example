//!
//! `From` 和 `Into`
//!
//! From 和 Into 本质上是互相转换的关系，
//! 比如 A类型可以通过B类型转换过来的话，那么也应该可以从A类型转换到B类型
//!

/// From 特性(trait) 允许我们使用一个明确的其他类型值来实例化一个当前的类型
/// 标准库提供的很多类型都实现了该特性，用于转换原始类型和公共类型
///
/// 比如 我们可以非常简单的使用 str 转换成 String 类型
/// ```
/// let my_str = "hello";
/// let my_string = String::from(my_str);
/// ```
///
fn example_from() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    // 通过实现指定类型的 From 特性的 from 方法可以很容易的实现通过指定类型构造实例
    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    let i = 0;

    // 因为实现了 From<i32> 特性，所以可以直接使用 into 方法来进行转换
    let num: Number = i.into();
    let num = Number::from(30);

    println!("My Number is {:?}", num);
}

/// Into 特性就是 From 的逆变实现，
/// 如果对应的类型已经实现了 From 特性的话，
///
/// 在使用 Into 的时候需要手动指定一个类型，因为编译器并不清楚你想要转换的目标类型是什么
///
fn example_into() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    // 如果指定的类型已经实现了 From 特性的话 就不用手动再次实现 Into 特性了
    impl Into<Number> for i32 {
        fn into(self) -> Number {
            Number { value: self }
        }
    }

    let int = 5;

    // 手动指定类型进行转换
    let num: Number = int.into();
    // 通过 Trait 对象，明确给出目标类型，调用转换方法
    let num = Into::<Number>::into(int);

    println!("My number is {:?}", num);
}

fn main() {
    example_from();
    example_into();
}
