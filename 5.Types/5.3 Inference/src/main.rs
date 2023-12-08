//!
//! Rust的智能类型推断引擎，可以根据代码的上下文来自动推断所需要的类型定义
//! 所以很多时候不需要手动声明类型，让推断引擎自动推断出类型
//!

fn main() {
    // 手动声明类型
    let elem = 5u8;

    // 创建一个 Vec 的容器
    let mut vec = Vec::new();
    // 在这里编译器并不知道vec是什么类型的，所以编译器先假定类型是 Vec<_> 类型
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    // 插入一下明确的类型
    vec.push(elem);
    // 编译器在这里发现我们插入了一个明确的类型，编译器就会把 vec 推断成 Vec<u8> 类型
    // TODO ^ 尝试注释上面这行代码看看 vec 的类型是什么

    println!("{:?}", vec);
}
