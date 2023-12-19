//!
//! [Rust Playground](https://play.rust-lang.org/) 是一个可以在线体验 Rust 代码的一个网页
//!
//! 使用 mdbook 让你的文档可以现在交互使用。
//!
//! [mdbook](https://github.com/rust-lang/mdBook) 可以让你的示例代码可以运行并且可以编辑
//!
//!
//! 还可以使用 `#[doc]` 宏来指定 `html_playground_url` 一个地址这个地址对应了一些测试代码，
//! 可以让你的代码可以在线交互测试
//!
//! 更多的相关属性可以[查看这里](https://doc.rust-lang.org/rustdoc/the-doc-attribute.html)
//!

///
///
/// 通过加上额外的标记 `rust,editable` 来让 mdbook 识别要生成交互的文档测试代码块
///
/// ```rust,editable
/// let result = add(1, 2);
/// assert_eq!(result, 3);
/// ```
///
fn add(a: usize, b: usize) -> usize {
    a + b
}

///
/// 通过添加 `ignore` 标记可以让 mdbook 来忽略这个代码块
///
/// ```rust,editable,ignore
/// let result = sub(10, 5);
/// assert_eq!(result, 5);
/// ```
fn sub(a: usize, b: usize) -> usize {
    a - b
}

fn main() {
    println!("Hello, world!");
}
