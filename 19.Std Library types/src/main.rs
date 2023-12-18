//!
//! 标准库
//!
//! Rust 标准库提供了很多基于原始类型的扩展类型，比较常用的是下面这些
//!
//! - 可变长的 `String` 类型： `"hello world"`
//! - 可变长的 `Vec` 类型：`[1,2,3]`
//! - 可选的 `Option` 类型： `Option<i32>`
//! - 用于错误处理的 `Result` 类型：`Result<i32, i32>`
//! - 基于堆内存动态分配的指针指针 `Box` 类型：`Box<i32>`
//!

///
/// 堆内存动态分配的智能指针 `Box`
///
/// 在 `Rust` 中数据默认都会保存在栈(`stack`)上，如果一个值被 `Box` 包装，则该数据
/// 会自动保存在堆上，`Box` 可以存放任意的类型，当 `Box` 走出作用域则相应的堆内存也会被回收。
///
/// 被装箱的值可以使用 `*` 解引用操作符来移除掉一层指针的引用。
///
fn box_() {
    use std::mem;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Point> {
        // 在堆上申请一块内存来保存 `Point` 实例，并且使用 `Box` 进行装箱，并返回 `Box`
        Box::new(Point { x: 0.0, y: 0.0 })
    }

    // 这里的类型标注都可以直接去掉，因为编译器能自动推断出类型
    // 这两个类型都是储存在栈上的
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // 这个是储存在堆上的
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // 这个也是储存在堆上的
    let boxed_point: Box<Point> = Box::new(origin());

    // 双重 `Box` 装箱
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // `Box` 占用的内存空间等于指针占用的内存空间
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // 从堆上复制数据到栈上，这里是复制不是转移所有权是因为 `Point` 实现了 `Copy` 和 `Clone`
    let unboxed_point: Point = *boxed_point;

    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}

///
/// 可变数组（vectors）
///
/// 可变数组和切片很相似，在编译阶段都不知道具体的长度是多少，
/// 而且可以在运行的过程中增加或减小长度，一个可变数组拥有三个关键的值
///
/// - 数据的指针
/// - 当前数组的长度 （就是当前有效数据的实际长度）
/// - 当前数组的容量 （是当前数组从堆内存中分配的长度，该长度有一定冗余，会根据有效数据的长度自动扩容或者缩容）
///
fn vectors() {
    // 迭代器可以转换成一个可变数组
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` 宏可以初始化一个任意长度数组
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // 向数组中添加数据
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // 错误: 不可变数组不能添加数据
    // collected_iterator.push(0);
    // TODO ^ 移除注释查看错误

    // `len` 方法可以告诉你当前的数组长度是多少
    println!("Vector length: {}", xs.len());

    // 数组可以通过方括号访问数据，索引值从 `0` 开始
    println!("Second element: {}", xs[1]);

    // `pop` 方法会返回并移除掉数组的最后一个元素
    println!("Pop last element: {:?}", xs.pop());

    // 如果访问的索引超过数组的有效数据数量则会崩溃
    // println!("Fourth element: {}", xs[3]);
    // TODO ^ 移除注释查看错误

    // 数组合一使用 `iter` 方法转换成迭代器使用 `for` 来迭代访问数组中的数据
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // 数组还可以利用迭代器的 `enumerate` 方法来对数据进行计数
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // 还可以使用 `iter_mut` 方法来生成可变引用的迭代器，这样就可以在迭代的过程中修改数组的内容。
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}

///
/// 在 `Rust` 中有两种字符串 `String` 和 `str`。
///
/// `String` 内部使用的是 `Vec<u8>` 类型保存字符串的每一个字节，
/// 同时保证了字符串一定是一个有效的 `UTF-8` 格式的序列，
/// `String` 因为使用了 `Vec<u8>` 所以数据是保存在堆中的是可变长的，并且不是以 `null` 为终止的。
///
/// `&str` 像是一个切片类似于 `&[u8]` 并且指向的内存一定是有效的 `UTF-8` 格式的序列，
/// 就像是对于 `String` 内部数据的引用，就像是 `&[T]` 是对 `Vec<T>` 的引用一样。
///
fn strings() {
    {
        // 所有的类型声明都是可以删除的，因为编译器能推断出类型是什么。
        // 所有的字面量都是对于程序中一段只读内存的引用。
        let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
        println!("Pangram: {}", pangram);

        // 把字符串转换成迭代器并且翻转字符串，这里并没有生成新的字符还只是对原始内存的引用。
        println!("Words in reverse");
        for word in pangram.split_whitespace().rev() {
            println!("> {}", word);
        }

        // `collect` 会对数据进行复制并生成一份新的数据。
        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort(); // 对数据进行排序
        chars.dedup(); // 对数据进行去重

        // 创建一个空的 `String` 类型
        let mut string = String::new();
        for c in chars {
            // 向字符串中添加单个字符
            string.push(c);
            // 向字符串中添加一段字符串
            string.push_str(", ");
        }

        // 对固定长度的 `[str, 2]` 数组创建一个不可变切片。
        let chars_to_trim: &[char] = &[' ', ','];
        // 对字符串数组进行 `trim_matches` 删除指定的首位字符，这里实际上只是对原始的字符串重新创建了一个引用的切片
        // 并没有生成任何新的字符串。
        let trimmed_str: &str = string.trim_matches(chars_to_trim);
        println!("Used characters: {}", trimmed_str);

        // 申请一块内存来存放字符串
        let alice = String::from("I like dogs");
        // 申请一块新的内存来存放变更后的字符串
        let bob: String = alice.replace("dog", "cat");

        println!("Alice says: {}", alice);
        println!("Bob says: {}", bob);
    }

    // 字面量和转义字符
    {
        // 你可以使用转义字符直接书写十六进制的字节码
        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        // 还可以写 `Unicode Code` 编码。
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!(
            "Unicode character {} (U+211D) is called {}",
            unicode_codepoint, character_name
        );

        // 字符串还可以是多行的
        let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
        println!("{}", long_string);
    }

    // 有时候在字符串中因为有太多的字符需要被转移所以提供了 `raw string` 的简便写法，
    // 所有在 `raw string` 中的字符串都不会被转义。
    {
        // 转义字符 `\` 在这个字符串中不生效
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // 如果你的字符中还包含了 `"` 符号的话，还可以使用 `#` 来包围字符串
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果你还需要 `#` 在你的字符串中出现，则还可以使用 `#s` 语法，其中 `s` 代表了重复书写 `#` 多少次，
        // 只要前后使用的是相同数量的标记则就认为是有效的字符串，你最多可以使用 65536 个 `#` 符号。
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }

    // 如果你使用的字符串格式不是 `UTF-8` 的 (`String` 和 `str` 都只能存放 `UTF-8`)，那你可以使用 `字节字符串(Byte strings)`。
    {
        use std::str;
        // Note that this is not actually a `&str`
        // 这个字面量并不是 `&str` 只是一个字节序列。
        let bytestring: &[u8; 21] = b"this is a byte string";

        // 字节数组并没有实现 `Display` 特性，所以打印的是原始的字节的数字。
        println!("A byte string: {:?}", bytestring);

        // 字节字符串可以使用转义字符来直接书写十六进制编码
        let escaped = b"\x52\x75\x73\x74 as bytes";
        // 但是不能使用 `Unicode Code` 编码
        // let escaped = b"\u{211D} is not allowed";
        println!("Some escaped bytes: {:?}", escaped);

        // `raw byte strings` 和 `raw strings` 一样
        let raw_bytestring = br"\u{211D} is not escaped here";
        println!("{:?}", raw_bytestring);

        // 转换字节数组到 `str` 类型，可能会失败因为字节数组的数据可能不是有效的 `UTF-8` 字符串
        if let Ok(my_str) = str::from_utf8(raw_bytestring) {
            println!("And the same as text: '{}'", my_str);
        }

        // 这里也是一样的效果
        let _quotes = br#"You can also use "fancier" formatting, \
                     like with normal raw strings"#;

        // 定义一个不是 `UTF-8` 格式的字节数组
        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

        // 这里会转换失败，因为上面的字节数组不是有效的 `UTF-8` 格式的
        match str::from_utf8(shift_jis) {
            Ok(my_str) => println!("Conversion successful: '{}'", my_str),
            Err(e) => println!("Conversion failed: {:?}", e),
        };
    }
    // 更多编码转换相关的可以[查看这里](https://crates.io/crates/encoding)
    // 更多关于字符串字面量和转义字符的相关详情可以[查看这里](https://doc.rust-lang.org/reference/tokens.html)
}

///
/// 很多时候我们更希望可以捕获到错误，而不是出现错误就让程序崩溃，
/// 可以通过使用 `Option` 来实现这个效果，`Option` 本身是一个枚举包含如下值
///
/// - None，表示失败或者没有值
/// - Some(value)，一个元组的结构，有一个 `value` 值可以是任意类型的
///
fn option() {
    // 数字除 `0` 返回一个 `None`，否则返回除法结果。
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    fn try_division(dividend: i32, divisor: i32) {
        // 通过 `match` 来处理 `Option` 的结果，执行特定代码。
        match checked_division(dividend, divisor) {
            None => println!("{} / {} failed!", dividend, divisor),
            Some(quotient) => {
                println!("{} / {} = {}", dividend, divisor, quotient)
            }
        }
    }

    try_division(4, 2);
    try_division(1, 0);

    // 绑定 `None` 到变量上，需要手动声明 `Option<T>` 的类型
    let none: Option<i32> = None;
    // 或者使用这个语法快捷绑定
    let _equivalent_none = None::<i32>;

    // 这里会自动推断类型
    let optional_float = Some(0f32);

    // 使用 `unwrap` 方法获取内部数据
    println!(
        "{:?} unwraps to {:?}",
        optional_float,
        optional_float.unwrap()
    );

    // 如果数据为 `None` 的话 `unwrap` 会调用 `panic!`
    // println!("{:?} unwraps to {:?}", none, none.unwrap());
    // TODO: 移除注释后运行查看错误
}

///
/// 我们看到了 `Option` 枚举可以用作返回值可以表示操作可能失败，但是 `None` 没有办法携带
/// 具体的失败原因，有时候我们需要明确的错误原因来告知用于为什么出现了错误，所以可以使用 `Result` 枚举。
///
/// `Result<T, E>` 拥有两个成员
///    
/// - Ok(value) 表示当前操作成功了，并且 `value` 是结果的值，`value` 是 `T` 类型的。
/// - Err(why) 表示当前的操作失败了，并且失败原因保存在了 `why` 中，`why` 是 `E` 类型的。
///
fn result() {
    mod checked {
        // 自定义错误
        #[derive(Debug)]
        pub enum MathError {
            DivisionByZero,
            NonPositiveLogarithm,
            NegativeSquareRoot,
        }

        pub type MathResult = Result<f64, MathError>;

        pub fn div(x: f64, y: f64) -> MathResult {
            if y == 0.0 {
                // 这里分母为 `0` 是不允许的所以抛出错误
                Err(MathError::DivisionByZero)
            } else {
                // 这里正常
                Ok(x / y)
            }
        }

        pub fn sqrt(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }

        pub fn ln(x: f64) -> MathResult {
            if x <= 0.0 {
                Err(MathError::NonPositiveLogarithm)
            } else {
                Ok(x.ln())
            }
        }
    }

    // `op(x, y)` === `sqrt(ln(x / y))`
    fn op(x: f64, y: f64) -> f64 {
        // 这是一个三层的 `match` 金字塔
        match checked::div(x, y) {
            Err(why) => panic!("{:?}", why),
            Ok(ratio) => match checked::ln(ratio) {
                Err(why) => panic!("{:?}", why),
                Ok(ln) => match checked::sqrt(ln) {
                    Err(why) => panic!("{:?}", why),
                    Ok(sqrt) => sqrt,
                },
            },
        }
    }

    // 这里会失败吗？
    // println!("{}", op(1.0, 10.0));
}

///
/// 当使用链条式的 `match` 来写代码会降低代码的可读性而且很混乱，
/// 幸运的使我们可以使用 `?` 操作符来简化这个过程，让代码重新变得可读。
///
/// `?` 操作符可以检查表达式返回的 Result 的值是否是 Err(T) 如果是的话
/// 会自动直接返回错误值，等于 `return Err(From::from(err))`，反之
/// 继续执行剩下的代码逻辑。
///
fn result_and_question_mark() {
    mod checked {
        #[derive(Debug)]
        enum MathError {
            DivisionByZero,
            NonPositiveLogarithm,
            NegativeSquareRoot,
        }

        type MathResult = Result<f64, MathError>;

        fn div(x: f64, y: f64) -> MathResult {
            if y == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(x / y)
            }
        }

        fn sqrt(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }

        fn ln(x: f64) -> MathResult {
            if x <= 0.0 {
                Err(MathError::NonPositiveLogarithm)
            } else {
                Ok(x.ln())
            }
        }

        // 这里使用 `?` 来优化代码逻辑
        fn op_(x: f64, y: f64) -> MathResult {
            // 如果 `div` 返回失败，则直接返回失败的结果。
            let ratio = div(x, y)?;

            // 如果 `ln` 返回失败, 则直接返回失败的结果。
            let ln = ln(ratio)?;

            sqrt(ln)
        }

        pub fn op(x: f64, y: f64) {
            match op_(x, y) {
                Err(why) => panic!(
                    "{}",
                    match why {
                        MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                        MathError::DivisionByZero => "division by zero",
                        MathError::NegativeSquareRoot => "square root of negative number",
                    }
                ),
                Ok(value) => println!("{}", value),
            }
        }
    }

    // checked::op(1.0, 10.0);
}

///
/// `panic!` 宏可以让当前线程退出并且展开调用堆栈，在此期间会把当前线程所拥有的所有对象都进行回收（会调用所有对象的析构函数 `drop`），
/// 当程序只有一个线程的时候，`panic!` 会打印当前的调用栈和错误信息，同时退出进程。
///
fn panic_() {
    // 重新实现除法
    fn division(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            // 这里直接调用 `panic!` 宏终止进程并提示错误信息
            panic!("division by zero");
        } else {
            dividend / divisor
        }
    }

    division(3, 0);

    println!("This point won't be reached!");
}

///  
/// 哈希表 (HashMap)
///
/// 数组和动态数组都使用整数来存取数据，`HashMap` 使用 `key` 进行存取数据，
/// `key` 可以是 `booleans(布尔值)`，`integers(整数)`，`strings(字符串)`，
/// 或者任意类型只要该类型实现了 `Eq` 和 `Hash` 特性的都可以。
///
/// `HashMap` 是一个可以动态扩容和缩容的容器，也可以手动通过 `HashMap::with_capcity` 指定一个初始容量的 `HashMap`
/// 或者使用 `HashMap::new()` 来创建一个空的 `HashMap`。 （更推荐创建空的哈希表）
///
fn hash_map() {
    use std::collections::HashMap;

    fn call(number: &str) -> &str {
        match number {
            "798-1364" => {
                "We're sorry, the call cannot be completed as dialed. 
                Please hang up and try again."
            }
            "645-7689" => {
                "Hello, this is Mr. Awesome's Pizza. My name is Fred.
                What can I get for you today?"
            }
            _ => "Hi! Who is this again?",
        }
    }

    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 通过 `key` 获取数据的引用
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` 当 `key` 不存在的时候返回 `None`，
    // 否则返回被更新的值。
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` 可以创建一个迭代器，这个迭代器可以访问哈希表中的所有数据。
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}

///
/// 哈希表自定义 `key` 类型
///
/// 一个类型只要实现了 `Eq` `Hash` 这两个特性就可以当做哈希表的 `key` 来使用，
/// Rust 内置可以直接当做 key 使用的类型如下
///
/// - `bool` 不常用因为只有两种可能。
/// - `int`，`uint` 所有整数的子类都可以使用，注意不包含 `f32` 和 `f64` 因为浮点数的精度问题会导致计算散列的时候出错。
/// - `String`，`&str` 字符串
///
/// 如果一个类型实现了 `Eq` 和 `Hash` 那么对应他的容器也会实现这两个方法，比如说
/// `Vec<T>` 这里 `T` 实现了 `Eq` 和 `Hash` 则 `Vec<T>` 也会自动实现 `Hash` 和 `Eq`
///
/// 可以通过属性宏 `#[derive(PartialEq, Eq, Hash)]` 非常简单的就能实现 `Eq` 和 `Hash` 特性，
/// 剩下的会由编译器自动实现相关的具体代码。
///
/// 下面这个示例只是演示怎么在哈希表中使用自定义类型来当做 `key` 而不是具体的怎么实现 `Hash` 特性。
///
fn alternate_custom_key_types() {
    use std::collections::HashMap;

    // `Eq` 要求必须实现 `PartialEq`。
    #[derive(PartialEq, Eq, Hash)]
    struct Account<'a> {
        username: &'a str,
        password: &'a str,
    }

    struct AccountInfo<'a> {
        name: &'a str,
        email: &'a str,
    }

    // 定义类型别名，使用自定义结构当做哈希表的 `key`
    type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

    fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
        println!("Username: {}", username);
        println!("Password: {}", password);
        println!("Attempting logon...");

        let logon = Account { username, password };

        match accounts.get(&logon) {
            Some(account_info) => {
                println!("Successful logon!");
                println!("Name: {}", account_info.name);
                println!("Email: {}", account_info.email);
            }
            _ => println!("Login failed!"),
        }
    }

    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    // 把结构和内容进行关联
    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}

///
/// 哈希集合（HashSet）
///
/// 可以把 `HashSet` 当做一个没有 `value` 的 `HashMap`，（底层也确实通过 `HashMap<T, ()>` 实现的）
/// 你可能很好奇这么做的原因是为了什么，毕竟已经有了动态数组 (`Vec<T>`)。
/// 这里最重要的原因是 `HashSet` 有一个特殊的特性就是可以保证容器内不包含重复的元素，但是 `Vec<T>` 做不到这一点。
/// 如果你尝试在 `HashSet` 中重复的设置一个值（新老值拥有同一个 `Hash` 值），那么这个新的值会直接替换掉老的值。
///
/// HashSet 还拥有四种集合的基本运算。
///
/// - `union` 并集 `A` 和 `B` 共同拥有的集合
/// - `defference` 差集 `A` 和 `B` 不想交的 `A` 的集合
/// - `intersection` 交集 `A` 和 `B` 共同拥有的集合
/// - `symmetric_difference` 对称差集 `A` 和 `B` 的并集中排除掉 `A` 和 `B` 交集的集合
///
///
fn hash_set() {
    use std::collections::HashSet;

    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 因为 `b` 中已经存在 `4` 了 所以 `HashSet::insert()` 返回 `false`
    // assert!(b.insert(4), "Value 4 is already in set B!");
    // TODO ^ 移除注释查看错误

    b.insert(5);

    // 如果容器内的类型实现了 `Debug` 则容器会自动实现 `Debug`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // `a` 和 `b` 的 并集 `[1, 2, 3, 4, 5]` 但是不保证顺序
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // `a` 和 `b` 的 差集 `[1]` 但是不保证顺序
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // `a` 和 `b` 的交集 `[2, 3, 4]` 但是不保证顺序
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // `a` 和 `b` 的对称差集 `[1, 5]` 但是不保证顺序
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}

///
/// `Rc` (引用计数 `Reference Counting`)
///
/// 让一个资源拥有多个拥有者，`Rc` 对象可以自动跟踪资源的引用计数，当引用技术归零则会自动销毁被引用的资源。
/// 当一个 `Rc` 资源被克隆一份则对象的引用计数就会增加1，如果被克隆的对象被销毁则引用计数自动减1，直到引用
/// 计数归零，则会自动销毁对应的资源。
///
/// 对 `Rc` 对象进行克隆，并不会深度拷贝数据，只是复制了一个新的 `Rc` 对象这个对象指向原始数据，并且增加对原始对象的引用计数。
///
fn rc_() {
    use std::rc::Rc;
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        // `Rc::new` 方法会转移参数的所有权
        let rc_a: Rc<String> = Rc::new(rc_examples);
        // 查询引用计数，这里会返回 1
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            // 通过 `Rc::clone` 方法复制一个新的引用对象，原始的引用计数 `+1`
            let rc_b: Rc<String> = Rc::clone(&rc_a);

            // 因为 `rc_a` 和 `rc_b` 是对同一个对象的引用使用的是同一个引用计数器，所以这里 `rc_b` 是 2， `rc_a` 也是 2
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // 这两个引用都是对同一个资源的 所以这两个资源是同一个
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // 因为 `Rc` 实现了 `Deref` 特性，所以这里访问的方法实际上是内部对象 `String` 的方法
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        // 这里 `rc_a` 是 1，因为 `rc_b` 已经因为作用域销毁被自动回收了，所以引用计数被 `-1`
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // 错误！`rc_examples` 因为已经被前一个作用域转移了所有权，所以这里不能再使用这个变量了。
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ 移除注释查看错误
}

///
/// Arc （Atomically Reference Counted）原子性引用计数器，多线程版本的引用计数器
/// 其引用计数的规则和 `Rc` 是一模一样的，只不过内部针对线程实现了一个线程之间的数据安全转移的特性。
/// 这个引用计数器专门为了多线程设计的，可以保证数据再多个线程之间安全转移。
///
fn arc() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    // 创建一个线程安全的数据引用
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // 通过 `clone` 方法给线程创建一个不可变引用，这个引用可以直接传递给线程。
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // 因为闭包声明的时候会自动捕获当前作用域下的外部变量使用 `move` 关键字让变量直接转移到线程中。
            println!("{:?}", apple);
        }); // 当线程执行完毕以后会自动回收掉所有引用的变量 这里会自动回收掉 `apple` 这个变量。
    }

    // 让当前线程等待1秒钟，好让所有的线程有时间能执行完内部的代码逻辑。
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    // `Box<T>` 堆内存动态分配的智能指针
    box_();

    // `Vec<T>` 可变数组
    vectors();

    // `String` 可变字符串
    strings();

    // `Option<T>` 枚举
    option();

    // `Result<T, E>` 枚举
    result();

    // `Result<T, E>` 和 `?` 表达式。
    result_and_question_mark();

    // 主动触发异常
    // panic_();

    // `HashMap<K, V>` 哈希表
    hash_map();
    // 自定义哈希表 `key` 类型
    alternate_custom_key_types();

    // `HashSet<T>` 集合
    hash_set();

    // `Rc<T>` 引用计数器
    rc_();

    // `Arc<T>` 多线程引用计数器
    arc();
}
