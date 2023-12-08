fn example01() {
    // 使用枚举类型对web事件进行分类。
    // 枚举的每一项都是独一无二的，枚举的每一项都可以包含任意数据
    // 但是枚举的每一项都属于该枚举类型
    // `PageLoad != PageUnload` 和 `KeyPress(char) != Paste(String)` 都是成立的

    enum WebEvent {
        // 枚举的每一项都是一个独立的
        PageLoad,
        PageUnload,
        // 使用类似元组结构来保存数据
        KeyPress(char),
        Paste(String),
        // 或者使用结构体保存数据
        Click { x: i64, y: i64 },
    }

    // 该函数接收 `WebEvent` 枚举，也就是说可以接受该枚举下定义的任意一项数据
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum` variant.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 通过克隆 `&str` 引用的数据 创建一新的 `String` 类型
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

fn example02() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                // 这里的 Self 就是一个类型别名，等于当前 impl 的类型
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    // 创建一个类型别名，类型别名并没有定义新的类型，只是用作缩短类型声明
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    // 类型别名可以有效的缩短类型声明
    let x = Operations::Add;
}

fn main() {
    example01();
    example02();
}
