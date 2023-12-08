//!
//! type 语句可以给已经存在的类型一个新的别名，
//! 类型别名必须使用大驼峰命名，否则编译器会给出警告，
//! 有一个例外情况就是语言内置的原始类型值没有这个警告，比如 usize，f32，等等
//!
//!
//! 类型别名通常是用来简化模版代码的，
//! 比如 std::io::Result<T> 就是 std::io::Result<T, std::io:Error> 的类型别名
//!

// `NanoSecond`, `Inch`, 和 `U64` 都是 `u64` 的类型别名
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // 类型别名没有提供任何的额外安全特性，
    // 类型别名也没有声明任何新的类型。
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
