//!
//! 输出文档
//!
//! `cargo doc` 可以输出当前工程的静态文档，该文档会自动保存在 `target/doc` 目录中。
//!
//! `cargo test` 可以运行当前工程中的所有测试用例，包括文档中的测试代码。
//!
//! `cargo test --doc` 可以只运行文档中的测试代码。
//!
//! 这些命令会自动的调用 `rustdoc` 和 `rustc` 来完成指定的操作。
//!
//!

///
/// 文档注释使用 `///` 开头，并支持 `Markdown` 语法
///
/// 大型的项目非常需要文档注释来解释某些具体的模块是什么样的，以及实现了什么功能，
/// 当使用 `rustdoc` 的时候，这些文档注释中的代码就会被编译成一个静态的文档，
///
fn doc_comments() {}

// #![crate_name = "doc"] // 当不使用 cargo 系统的时候可以使用这个宏来声明包（crate）的名称.

/// `人` 类型的定义
pub struct Person {
    /// 一个人必须拥有一个名字
    name: String,
}

impl Person {
    /// 返回一个人的实例，这个人拥有给定的名字
    ///
    /// # 参数
    ///
    /// * `name` - 一个名字，这个名字会给到一个具体的人
    ///
    /// # 示例
    ///
    /// ```
    /// // 你可以在这里写 Rust 的代码
    /// // 如果你使用 rustdoc --test 来查看文档的话还会自动的运行这段代码来测试你的实现是否正常。
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 和朋友们说 `hello`
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

#[doc(inline)] // 这个宏可以让文档嵌入到其他使用方的文档中
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}

#[doc(no_inline)] // 这个宏禁止文档被嵌入到任何页面中
pub use crate::mem::drop;

#[doc(hidden)] // 这个宏会让文档隐藏不显示
pub use self::async_await::*;

fn main() {
    println!("Hello, world!");
}
