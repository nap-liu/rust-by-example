//!
//! `crate(包)` 是一个 Rust 的编译单元，不管什么时候使用 `rustc some_file.rs`，
//! 都会把输入的文件 `some_files.rs` 当做一个 `crate(包)`，
//! 如果输入的文件中有 `mod` 关键字，则会尝试把对应的声明文件的内容插入到 `mod` 关键字中，
//! 当所有的 `mod` 关键字都处理完了以后，编译器才开始进行编译，
//!
//! 也就是说单独的模块是不能进行编译的，只有 `crate(包)` 才能进行编译。
//!
//! 一个 `crate(包)` 可以编译成可执行的二进制文件也可以编译成一个库(library)文件，
//! 默认情况下 `rustc` 会把 `crate(包)` 编译成可执行的二进制文件，可以通过额外的
//! 命令行参数 `--crate-type` 参数指定为 `lib` 来修改这个行为。
//!

// 创建一个简单的库(library)
// 然后使用 `rustc --crate-type lib rary.rs` 编译成一个库，
// 编译完成后会自动生成一个 `library.rlib` 的文件，该文件就是生成好的库文件，
// 编译输出的文件名可以通过 `--crate-name` 来指定。

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
