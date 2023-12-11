//!
//! 幽灵类型 （Phantom type）
//!
//! 幽灵类型的特性是只存在于编译阶段用于辅助编译器推断类型和强制保持某些变量，属性的生命周期。
//!
//! 有很多时候需要和外部的三方库交换数据，这时候我们把不安全的代码封装到安全的结构或者代码中的时候，
//! 需要交换很多数据，这些数据由三方的库申请的，这时候编译器并不知道这个数据的生命周期，所以需要这个
//! 幽灵类型来明确的告知编译器这个类型的生命周期，这样编译器才能通过编译检查。
//!
//! 关于幽灵类型更加详细的解释可以看[这里](https://doc.rust-lang.org/nomicon/phantom-data.html)
//!
//! 幽灵定义在泛型中，通过标准库导出的特殊类型 `std::marker::PhantomData` 来标记一个幽灵类型，
//! 该类型只用于编译器的类型检查，并不占用空间，也不能用于存放数据，也没有运行时的行为。
//!
//!

use std::marker::PhantomData; // 导入幽灵类型

// 声明一个拥有幽灵类型的元组结构，两个泛型 `<A, B>` 泛型 `A` 拥有是常规的类型，泛型 `B` 是一个隐藏的幽灵类型。
#[derive(PartialEq)] // 允许结构直接行进 `==` 运算
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 声明一个拥有幽灵类型的结构体，两个泛型 `<A, B>` 泛型 `A` 拥有是常规的类型，泛型 `B` 是一个隐藏的幽灵类型。
#[derive(PartialEq)] // 允许结构直接行进 `==` 运算
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 注意：代码编译的时候只会为正常的泛型 `A` 申请空间，而幽灵类型 `B` 没有空间，
//      所以使用了幽灵类型 `B` 的字段或者位置都不能当做实际存在的类型使用。

fn main() {
    // 这里的 `f32` 和 `f64` 类型是给幽灵类型使用的。
    // PhantomTuple type specified as `<char, f32>`.
    // 明确指定泛型的数据类型，这里声明类型为 `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 明确指定泛型的数据类型，这里声明类型为 `<char, f64>`
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 明确指明类型 `<char, f32>`
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 明确指明类型 `<char, f64>`
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译错误！类型匹配不能对比
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // 编译错误！类型匹配不能对比
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
