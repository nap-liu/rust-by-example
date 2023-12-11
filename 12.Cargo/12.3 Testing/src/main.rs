//!
//! 集成测试和单元测试
//!
//! 对于任何软件都一样，集成测试和单元测试都是非常重要的一环，可以用来保证软件的质量。
//! `Rust` 中对于单元测试和集成测试是第一优先级支持的事情。
//!
//! 对于单元测试一般都会直接放在模块中，而集成测试则会放在根目录下的 `tests` 目录中。
//!
//! 文件结构如下
//!
//! ```sh
//! foo
//! ├── Cargo.toml
//! ├── src
//! │   └── main.rs
//! │   └── lib.rs
//! └── tests
//!     ├── my_test.rs
//!     └── my_other_test.rs
//! ```
//!
//! 可以使用 `cargo tests` 来执行整个工程的测试，该命令会先执行`单元测试`，通过了以后会执行 `集成测试`。
//! 还可以使用额外的参数来指定执行某些测试 `cargo test test_foo`，该命令指只会执行匹配 `test_foo` 的测试代码。
//!
//! `cargo` 运行测试的时候是并行的，所以需要注意一点的是测试的代码不能出现竞争关系。
//!
fn main() {
    println!("Hello, world!");
}

/// 可能会出现竞争关系的测试代码。
/// 这个测试运行完成以后，文件的内容预期是
/// ```text
/// Ferris
/// Ferris
/// Ferris
/// Ferris
/// Ferris
/// Corro
/// Corro
/// Corro
/// Corro
/// Corro
/// ```
///
/// 但是实际上可能是这样的
/// ```text
/// Corro
/// Ferris
/// Corro
/// Ferris
/// Corro
/// Ferris
/// Corro
/// Ferris
/// Corro
/// Ferris
/// ```
#[cfg(test)]
mod tests {
    // 导入使用的模块
    use std::fs::OpenOptions;
    use std::io::Write;

    // 写文件
    #[test]
    fn test_file() {
        // 打开 `ferris.txt` 文件，如果文件不存在则创建文件。
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // 向文件中写入5次 `Ferris`
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }

    // 写上一个测试同一个文件
    #[test]
    fn test_file_also() {
        // 同上
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // 向文件中写入5次 `Corro`
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }
}
