//! 死代码警告
//! 编译器会在编译的过程中对没有使用的代码打印警告，
//! 我们可以使用编译器提供的 `dead_code` 属性来禁用警告，
//!
//! 在真实的工程中，你应该解决死代码的问题，而不是使用属性来禁用警告！！！
//!

fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用掉 `dead_code` 的代码检查
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ 尝试添加属性来禁用警告

fn main() {
    used_function();
}
