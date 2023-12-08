// 这个结构体不能被 `fmt::Display` 或 `fmt::Debug` 打印
struct UnPrintable(i32);

// `derive` 属性为当前的结构体自动实现了 `fmt::Debug` 的打印方法
#[derive(Debug)]
struct DebugPrintable(i32);

// derive宏为 `Structure` 自动实现了 `fmt::Debug`。
// `Structure` 是一个结构体，这个结构体内部保存了一个 `i32` 类型的值。
#[derive(Debug)]
struct Structure(i32);

// 让 `Deep` 结构体嵌套 `Structure` 结构体，同时使用 `derive` 宏让 `Deep` 也是可打印的
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // 使用 `fmt::Debug` 的标记 `{:?}` 和 `fmt::Dispaly` 的 `{}` 标记很像。
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` 结构体是可打印的
    println!("Now {:?} will print!", Structure(3));

    // 有一个小问题是 `derive` 宏打印出来的内容格式是不可控的。
    // 如果我们只想打印出 `7` 应该怎么办?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化版的输出，带空格和缩进
    println!("{:#?}", peter);
}
