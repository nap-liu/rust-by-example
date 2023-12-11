//!
//! 使用库文件
//!
//! 想要让包连接到一个库，需要使用 rustc 的 --extern 的参数来指定库名称，
//! 比如 `rustc using_library.rs --extern rary=library.rlib`，
//! 通过参数指定的库会绑定通过指定的名字当做一个模块导入到当前做的作用域中。
//! 编译以后会生成一个 `using_library` 的二进制可执行文件。
//!

// Rust 2015版本之前需要明确的导入需要使用的外部库
// extern crate rary;

fn main() {
    rary::public_function();

    // 错误！`private_function` 是私有函数。
    //rary::private_function();

    rary::indirect_access();
}
