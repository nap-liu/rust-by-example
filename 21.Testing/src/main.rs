//!
//! Rust 非常注重正确性并且语言支持内部直接写测试代码。
//!
//! 测试分为了三种类型
//! - 单元测试（Unit testing）
//! - 文档测试 (Doc testing)
//! - 集成测试 (Integration testing)
//!
//! Rust 还支持特殊的依赖项，专门为测试使用的依赖。
//! 测试使用的依赖项 (Dev-dependencies)
//!

/// 测试是 `Rust` 的函数，用来测试一些非测试代码的正确性，有些测试函数是为了初始化一些环境，
/// 在测试函数中可以使用断言来验证结果是否符合预期
///
/// 大多数单元测试都是在 `tests` 模块中定义的，这个模块使用属性宏 `#[cfg(test)]` 来标记模块，
/// 测试函数使用 `#[test]` 宏来标记是一个测试函数。
///
/// 测试中可以使用断言来触发 `panic!`，下面是常用的一些断言宏。
///
/// - `assert!(expression)` 表达式为 `false` 的时候触发 `panic!`。
/// - `assert_eq!(left, right)` 和 `assert_nq!(left, right)` 左右两个值不相等的时候触发 `panic!`。
///

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 这个是一个错误的加法实现，用于测试失败的情况。
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

// 求平方根
#[allow(dead_code)]
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

///
/// 定义单元测试模块，用于测试内部实现的代码逻辑。
///
/// 可以使用附加的参数来运行指定的测试
/// - `cargo test test_any_panic` 只运行 `test_any_panic` 单元测试
/// - `catgo test -- --ignored` 只运行包含 `#[ignore]` 的单元测试
///
#[cfg(test)]
mod tests {
    // 使用 `super` 关键字来把父级作用域下的方法都引入到当前模块中。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic]
    fn test_bad_add() {
        // 这里使用故意写错的函数来测试失败的情况
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
        assert_eq!(add(2, 100), 102);
    }

    #[test]
    #[ignore] // 这个测试默认是不会运行的
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        // 使用 `?` 操作符快捷返回出错的情况
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic] // 使用 `should_panic` 宏来标记 函数应该要触发 `panic!`
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")] // 给 `should_panic` 传递参数，来表示 `panic!` 的信息应该包含指定的内容。
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}

///
/// 有很多时候会需要引用一些模块是专门为了测试时候使用的（用于代码示例，或者用于基准测试），
/// 这些模块可以定义在 `Cargo.toml` 的 `[dev-dependencies]` 这个字段下，
///
/// ```toml
/// [package]
/// name = "testing"
/// version = "0.1.0"
/// edition = "2021"
/// [dev-dependencies]
/// pretty_assertions = "1"
///
/// ```
///
#[cfg(test)]
mod tests_dependencies {
    use super::*;
    // 这个模块只能在测试代码中使用，其他的非测试代码不能使用。
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5)
    }
}

fn main() {
    println!("Hello, world!");
}
