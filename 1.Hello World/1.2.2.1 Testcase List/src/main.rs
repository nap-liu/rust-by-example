use std::fmt;

// 定义 `List` 结构体，内部包含一个 `Vec`
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 通过元组的索引，获取内部 `Vec` 的引用
        let vec = &self.0;

        // 不管有没有数据都要先打印一个开始括号
        // `?` 操作符表示如果 `write!` 宏出错了，则结束当前的函数，并把错误返回给调用方。
        write!(f, "[")?;

        // 使用 `Vec` 的 `iter()` 方法获取值引用的迭代器，再使用迭代器 `enumerate()` 返回一个带有计数器的迭代器
        for (count, value) in vec.iter().enumerate() {
            // 除了第一个值以外的所有值都先在前面打印一个 `", "` 分隔符
            if count != 0 {
                write!(f, ", ")?;
            }
            // 然后继续打印当前的数据
            write!(f, "{}", value)?;
        }
        // 最后补全结束的括号
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
