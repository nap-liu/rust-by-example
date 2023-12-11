//!
//! 构建脚本
//! 有些时候仅仅使用 `cargo build` 是不够的，有可能你依赖了一些其他的东西，
//! 库或者需要动态生成一些代码，或者需要预编译一些其他的代码，才能让工程能正常编译。
//!
//! 所以 `cargo` 提供了一个构建脚本的能力，可以通过构建脚本来让 `cargo` 在开始编译
//! 工程以前先执行某些特定的操作。
//!
//! 你可以在 `Cargo.toml` 文件的 `[package]` 中添加一个 `build = "build.rs"`
//! 属性来指定执行 `build.rs`，来做一些其他的操作，如果没有指定这个字段的话，
//! `cargo` 会默认使用项目根目录下的 `build.rs` 文件。
//!
//! 构建脚本和其他的 `Rust` 代码一样，只不过 `cargo` 会在工程编译之前先执行构建脚本。
//!
//! 你可以在构建脚本中使用一些 `cargo` 预定义好的环境变量，这些变量可以在[这里](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)找到
//! 构建脚本在执行过程中输出的内容可以在 `target/debug/build/<pkg>/output` 中找到。
//! 输出的内容中使用 `cargo:` 开头的内容会被 `cargo` 直接解析，通过这些命令可以定义 `cargo` 的一些行为。
//!
//! 完整的指令列表可以在[这里](https://doc.rust-lang.org/cargo/reference/build-scripts.html)找到
//!
fn main() {
    println!("Hello, world!");
}
