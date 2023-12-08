//!
//! 字面量数字可以通过添加类型后缀来指定变量类型，
//! 比如说 字面量数字 42 是 i32 类型，可以写成 42i32 来声明 42 是 i32 类型的
//!
//! 如果数字字面量没有明确指明类型，编译器会自动根据使用的地方自动推断合适的类型，
//! 如果没有地方使用的话，整数型会默认推断为 i32，浮点数会自动推断为 f64
//!

fn main() {
    // 带后缀的声明
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 没有后缀的类型声明，编译器会根据变量的使用自动推断合适的类型，如果无法推断则使用默认类型
    let i = 1; // 整数默认类型 i32
    let f = 1.0; // 浮点数默认类型 f64

    // `size_of_val()` 方法返回变量使用了多少个字节（1个字节使用8个比特位）
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
