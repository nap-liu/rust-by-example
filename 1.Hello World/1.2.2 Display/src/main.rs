use std::fmt::write;

fn example01() {
    // 使用 `use` 关键字把 `fmt` 模块引入到当前作用域。
    use std::fmt;

    // 定义一个结构体然后实现 `fmt::Display` 特性。
    // 当前定义的结构体是一个 `Tuple (元组)`，改元组包含一个 `i32` 的值
    struct Structure(i32);

    // 想要通过 `{}` 标记格式化结构，必须手动实现 `fmt::Display` trait(特性)
    impl fmt::Display for Structure {
        // 这个特性要求必须实现 `fmt` 方法，该方法签名如下
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 第一个参数是当前的结构体示例，第二个参数是输出的目标缓冲区(Buffer)。
            // 返回值是 `fmt::Result`，这个值表示格式化的结果。
            // `write!` 宏是和 `println!` 宏非常相似，只不过 `write!` 宏允许向指定向哪个缓冲区（Buffer）写出数据。
            write!(f, "{}", self.0)
        }
    }

    let demo = Structure(100);

    println!("struct display: {}", demo);
}

fn example02() {
    use std::fmt; // 导入 `fmt`

    // 该结构体内部有两个值，`derive(Debug)` 宏会自动实现 `fmt::Debug`方法。
    // 我们自己实现 `fmt::Display` 方法来对比两种实现的结果。
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // 手动为 `MinMax` 实现 `fmt::Display` 特性
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // 使用 `self.number` 来引用对应位置的值，输出到结果缓冲区（Buffer）中。
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // 定义一个结构体，包含名字和值，用来对比不同结构下显示的内容
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // 类似的为 `Point2D` 结构体实现 `Display`
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 自定义输出格式，只显示 `x` 和 `y`
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 下面这个例子会出现错误，因为我们只实现了 `Debug` 和 `Display`
    // 但是 `{:b}` 需要额外实现 `fmt:Binary` 特性。
    // println!("What does Point2D look like in binary: {:b}?", point);
}

fn main() {
    example01();
    example02();
    println!("Hello, world!");
}
