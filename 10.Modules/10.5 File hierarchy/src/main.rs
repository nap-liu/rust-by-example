//!
//! 通过文件结构组织模块
//!
//! Rust 的模块可以按照文件的结构来组织，
//! 目录会当做一个子模块的查找路径。
//!

// 这个定义会让 `Rust` 去尝试查找 `my.rs` 或者 `my/mod.rs` 这个文件，并把该文件当做一个模块。
// `my` 模块的嵌套模块可以创建一个 `my` 的目录，把子模块放进去就可以了。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    // 调用 `my` 模块中的方法。
    my::function();

    // 调用全局作用域下的方法。
    function();

    // 再次调用 `my` 模块中的方法。
    my::indirect_access();

    // 调用 `my` 模块中嵌套的子模块的方法。
    my::nested::function();
}
