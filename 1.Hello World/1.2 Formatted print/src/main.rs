fn main() {
    // 通常情况下，`{}` 会自动替换成任意的参数。这些参数会被转换成字符串。
    println!("{} days", 31);

    // 可以在 `{}` 中使用附加参数的序号来指定使用哪个参数来替换 `{}`。
    // 附加参数是第一个格式化字符串后面跟随的参数，附加参数的索引从0开始计算。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 附加参数可以手动指定名称。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `{}` 中使用 `:` 来指定转换的格式。
    println!("Base 10 (十进制):      {}", 69420); // 69420
    println!("Base 2 (二进制):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (八进制):       {:o}", 69420); // 207454
    println!("Base 16 (小写十六进制): {:x}", 69420); // 10f2c
    println!("Base 16 (大写十六进制): {:X}", 69420); // 10F2C

    // 可以指定固定宽度，并且从左侧自动填充空白文本
    // 下面这个例子会输出 "    1"，（四个空格和一个"1"，总长度是5）。
    println!("{number:>5}", number = 1);

    // 可以手动指定一个字符用于填充剩余长度。
    println!("{number:0>5}", number = 1); // 00001
                                          // 同样可以通过使用 `<` 来使用右侧填充，这样的话输出就是 "10000"。
    println!("{number:0<5}", number = 1); // 10000

    // 还可以使用命名参数来指定填充长度。
    // 只需要在命名参数后面添加 `$` 符号来使用命名参数。
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust还会检查你传入的参数数量和格式化字符串中使用的数量是否匹配。
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ 添加缺失的参数: "James"

    // 只有实现了 fmt::Display 的类型才能通过 `{}` 来格式化
    // 用户自定义的类型默认没有实现 fmt::Display.

    #[allow(dead_code)] // 禁用 `dead_code` 未使用代码的编译警告
    struct Structure(i32);

    // 因为 `Structure` 没有实现 fmt::Display 所以不能被编译。
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ 尝试取消注释查看编译错误。

    // 从 Rust 1.58 版本开始，你可以直接在格式化字符串中使用变量。
    // 这个会输出 "    1"，4个空格和一个 "1"。
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
