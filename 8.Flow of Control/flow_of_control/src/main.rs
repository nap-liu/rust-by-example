//! 控制流

/// 分支控制 if-else 和大多数语言一样，
/// 但是 不需要使用 () 来包围逻辑表达式
/// 每一个分支都使用一个 {} 来包围语句块，
/// 因为 if-else 本身也是一个表达式，所以要求所有分支返回相同类型的值
fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // 表达式返回 `i32` 类型的值.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // 这里也需要返回 `i32` 类型的值.
        n / 2
        // TODO ^ 通过添加`;`来让语句强制返回 `()` 看看会发生什么.
    };
    //   ^ 因为这里使用了 let x = if {} else {} 的语法，所以这里必须使用 `;` 结尾.

    println!("{} -> {}", n, big_n);
}

/// Rust 提供了一个关键字 `loop` 来实现一个无限循环的语句
/// 可以通过 `continue` 和 `break` 关键字来控制循环的跳过和终止
fn loop_() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环的语句
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过当前的循环，直接执行下一次循环
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 使用 `break` 关键字手动退出循环
            break;
        }
    }
}

/// 循环语句也可以通过使用 `label` 来标记循环体
/// 通过 `continue` 和 `break` 传递标记的 `label` 来控制循环的执行
fn nesting_and_labels() {
    #![allow(unreachable_code, unused_labels)]

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 不给 break 传递参数的话 默认只会退出当前最近一层的循环
            //break;

            // 如果传递了 label 标记 则会直接退出标记的循环，不关心循环嵌套了多少层
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

/// 同样的 `loop` 也可以作为语句使用
/// 通过 `break` 传递语句的最终返回值
fn returning_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("loop result is: {}", result);
}

/// while 关键字提供了一个表达式为 true 就不断重复执行代码块的能力
fn while_() {
    // 定义计数变量
    let mut n = 1;

    // 当 n < 101 的时候就会不断地执行语句块的代码
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 增加计数器
        n += 1;
    }

    println!("n is: {}", n);
}

/// for in 可以使用 `Iterator（迭代器）` 来不断地从迭代器中每次提取一个数据，
/// 在 Rust 中迭代器非常容易创建，可以通过 `a..b` 来快捷的创建一个从 a 开始(包含a)一直到 b (不包含b) 的迭代器
fn for_and_range() {
    // n 会从 0 开始一直到 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 迭代器还可以使用 `a..=b` 的语法，这个迭代器表示从 a 开始（包含a）一直到 b 结束（包含b）
    // `n` 会从 1 开始一直到 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

/// for in 语法还可以通过很多种方法迭代器(Iterator)，迭代器特性还可以通过手动实现 Iterator 特性来实现
/// for in 语句在默认的情况下会调用 into_iter() 来把连续的集合转换为迭代器
///
/// 最常用的三种是，into_iter(), iter(), iter_mut()
fn for_and_iterators() {
    // iter() 方法
    // 这个方法是对原始数据创建一个数据借用的迭代器，每一次迭代都能获取到当前元素的借用值
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    // 因为使用的是借用迭代 所以 names 还能继续使用
    println!("names: {:?}", names);

    // into_iter() 方法
    // 该方法会直接转移元素的所有权到迭代器中，
    // 调用完该方法以后，原始的变量就不能再次使用了，因为所有权转移了
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    // FIXME ^ 注释这行代码 因为 into_iter() 方法转移所有权到迭代器内部了 所以 names 不能使用了

    // iter_mut() 方法
    // 该方法会创建一个可变借用的迭代器，也就是在迭代过程中可以对原始数据进行修改，
    // 因为是借用关系所以迭代以后，原始变量依旧有效
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

/// Rust 通过 `match` 关键字提供了模式匹配的能力
/// 该模式类似于 C 的 `switch` 语句，第一个匹配的模式代码会被执行，
/// 并且必须所有可能的情况都要覆盖到。
fn match_() {
    let number = 13;
    // TODO ^ 尝试其他的整数值

    println!("Tell me about {}", number);
    match number {
        // 匹配固定的一个数字 1
        1 => println!("One!"),
        // 匹配多种可能得数字
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ 尝试把数字 13 添加到质数列表中
        // 匹配一个固定的区间范围
        13..=19 => println!("A teen"),
        // 处理剩余其他可能的值
        _ => println!("Ain't special"),
        // TODO ^ 尝试注释掉最后的这个分支
    }

    let boolean = true;
    // 还可以匹配 bool 表达式
    let binary = match boolean {
        // 匹配的模式必须覆盖到所有可能的结果
        false => 0,
        true => 1,
        // TODO ^ 尝试注释掉一个分支
    };

    println!("{} -> {}", boolean, binary);
}

// match 关键字还可以匹配元组
fn match_tuples() {
    let triple = (0, -2, 3);
    // TODO ^ 尝试不同的元组值

    println!("Tell me about {:?}", triple);
    // match 表达式中可以对元组进行解构
    match triple {
        // 第一个元素是 0 的情况下，把第二个值命名为 y, 第三个值命名为 z
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` 关键字可以忽略任意数量的元组值
        _ => println!("It doesn't matter what they are"),
        // `_` 关键字表示匹配任意的值
    }
}

//  match关键字还可以匹配数组和切片
fn match_array_slice() {
    // 尝试修改数组，或者把数组换成切片
    let array = [1, -2, 6];

    match array {
        // 当第一个元素是0，把第二个元素命名为 second， 第三个元素命名为 third
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // 当第一个元素是0，忽略第二个元素，把第三个元素命名为 third
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // 当第一个元素是-1，把第二个元素命名为 second，忽略后面的所有值
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 这个分支不能被编译，因为没有正确的匹配上数组的数量
        // [-1, second] => println!("The code below would not compile"),

        // 当第一个元素是3，把第二个元素命名为 second，把剩余的所有数据收集到一个数组或切片中（类型取决于传递进来的数据类型）
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // 合并上面的用法到一起
        // 绑定 第一个元素到 first，最后一个元素到 last，中间剩余的所有的放到 middle 中
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

// 匹配枚举值
fn match_enum() {
    // 使用 `allow` 宏禁用未使用的代码警告
    #[allow(dead_code)]
    enum Color {
        // 这三个值只定义名字
        Red,
        Blue,
        Green,
        // 用来表示颜色不同数据结构，都使用 u32 类型来保存颜色信息
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // TODO ^ 尝试不同类型的枚举值

    println!("What color is it?");
    // 枚举可以通过 `match` 关键字来解构
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // 因为枚举中的所有项都已经匹配完了，不需要再写其他的匹配项了
    }
}

/// 指针和引用
/// 对于指针来说，不像是在C/C++中使用方式，`解构`和`解引用`是两个不同的概念
///
/// 解引用使用的是 `*`
/// 解构使用的是 `&`，`ref` 和 `ref mut`
///
fn match_points_or_ref() {
    // 这里声明的是一个 i32 类型的引用值，也就是说 reference是 `&i32` 类型
    let reference = &4;

    match reference {
        // 如果 `reference` 和 `&val` 匹配的话，匹配的过程应该是这样
        // &val = &i32 因为都有相同的 `&` 符号，所以相同的符号被移除掉，带入数值的计算过程如下
        // &val = &i32 => &val = &4 => val = 4
        // 所以最终val的值就是 4
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果想避免在 match 的匹配模式中使用 `&` 的话，可以提前进行解引用
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 如果不想声明引用则声明的时候可以不写引用`&`符号
    let _not_a_reference = 3;

    // Rust 提供了另外一个操作符来让一个非引用的值强制转换成引用值
    // 通过在声明变量前面添加 `ref` 关键字，强制让变量是一个引用
    // 该方法在 match 语句中经常使用
    let ref _is_a_reference = 3;

    // 如果一个变量不是引用，但是可以通过添加 `ref` `ref mut` 强制让他转换成一个引用
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字创建一个不可变引用
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 使用 `ref mut` 关键字创建一个可变引用.
    match mut_value {
        ref mut m => {
            // 得到一个可变的引用类型，通过解引用来修改原始值
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

// match 还可以匹配结构体
fn match_struct() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // 修改结构体中的数据，看看会发生什么
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // 结构的过程可以对字段重命名，解构中字段的书写顺序不会影响结果
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // 也可以使用 `..` 关键字忽略一些不需要的字段
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // 下面这个分支会报错，因为结构的时候并没有覆盖所有的字段
        // Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // 还可以使用 let 语句直接对进行解构
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
}

// 增加 match 分支的匹配条件
fn match_guards() {
    {
        #[allow(dead_code)]
        enum Temperature {
            Celsius(i32),
            Fahrenheit(i32),
        }

        let temperature = Temperature::Celsius(35);
        // ^ TODO 尝试 Temperature 其他的枚举值

        match temperature {
            Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
            // 当上面的条件不命中的时候就会走到下面这个分支中
            Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

            Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
            Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
        }
    }

    {
        let number: u8 = 4;

        // 编译器检查match的条件是否覆盖了所有可能的时候会忽略掉额外的条件判断
        match number {
            i if i == 0 => println!("Zero"),
            i if i > 0 => println!("Greater than zero"),
            _ => unreachable!("Should never happen."),
            // TODO ^ 取消上面这行注释
        }
    }
}

// 动态值绑定
fn match_binding() {
    {
        // `age`函数返回一个u32类型的值
        fn age() -> u32 {
            15
        }
        println!("Tell me what type of person you are");

        match age() {
            0 => println!("I haven't celebrated my first birthday yet"),
            // 绑定区间的时候最大的问题就是，命中了区间确无法知道具体命中的值是多少
            // @ 操作符可以让你把命中区间的值绑定到一个变量上
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            // 这里不是区间值所以不需要重新绑定. Return the result.
            n => println!("I'm an old person of age {:?}", n),
        }
    }

    {
        fn some_number() -> Option<u32> {
            Some(42)
        }
        match some_number() {
            // 同样的也可以使用在任意的枚举值中
            // 当值等于 42 的时候，n 等于 42 有点脱裤子放屁了
            Some(n @ 42) => println!("The Answer: {}!", n),
            // 任意的其他值绑定到 n 上
            Some(n) => println!("Not interesting... {}", n),
            // None 走到兜底匹配
            _ => (),
        }
    }
}

fn if_let() {
    // 基础的 match 表达式
    {
        // 创建一个 Option<i32> 类型
        let optional = Some(7);

        match optional {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
                // 使用 match 关键字 解构 Option 中的数据
            }
            _ => {} // 这个统配的分支存在是因为 match 操作符必须要处理所有的可能结果
        };
    }

    // 使用 `if let` 表达式尝试解构并执行一些语句
    // 一旦使用了 `if let` 表达式，编译器就不会像 `match` 操作符一样做所有情况的检查，也就可能导致某些情况没有覆盖到
    {
        // 全都定义为 `Option<i32>`
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;

        // `if let` 解构的语义化读法是，如果 `number` 可以被 `Some(i)` 解构的话，则执行对应的 {} 代码块
        if let Some(i) = number {
            println!("Matched {:?}!", i);
        }

        // 如果需要知道结构失败的情况的话直接添加 `else {}` 分支即可
        if let Some(i) = letter {
            println!("Matched {:?}!", i);
        } else {
            // 如果解构失败的话执行这些代码
            println!("Didn't match a number. Let's go with a letter!");
        }

        // 解构失败以后的额外判断条件
        let i_like_letters = false;

        if let Some(i) = emoticon {
            println!("Matched {:?}!", i);
        // 如果解构失败的话，会继续尝试下一个 `else if` 的逻辑判断
        // 在这里依旧会失败因为 `i_like_letters` 的值是false
        } else if i_like_letters {
            println!("Didn't match a number. Let's go with a letter!");
        } else {
            // 最后会走到 `else {}` 分支中
            println!("I don't like letters. Let's go with an emoticon :)!");
        }
    }

    // 同样的 `if let` 表达式还可以尝试解构 `枚举(enum)`
    {
        // 用于
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        // 创建几个枚举变量
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        // `使用 if let` 解构变量，因为 `a` 就是 `Foo:Bar` 类型，所以会执行 {} 中的语句
        if let Foo::Bar = a {
            println!("a is foobar");
        }

        // 变量 `b` 不能解构为 `Foo::Bar`
        // 所以 {} 中的语句不会被执行
        if let Foo::Bar = b {
            println!("b is foobar");
        }

        // 变量 `c` 可以解构为 `Foo::Qux(value)`
        // 类似于 `if let Some(value) = x {}` 一样
        if let Foo::Qux(value) = c {
            println!("c is {}", value);
        }

        // 特定的 value 值限定也是支持的
        // 下面是支持的三种语法特定值的限定语法
        if let Foo::Qux(value @ 100) = c {
            println!("c is one hundred");
        }
        if let Foo::Qux(value @ (0..=100)) = c {
            println!("c is one hundred");
        }
        if let Foo::Qux(value @ (0 | 100)) = c {
            println!("c is one hundred");
        }
    }

    // `if let` 特性还有一个好处就是可以不用实现 `PartialEq` 特性就能判断类型是否一致
    {
        // 这个枚举没有实现 `PartialEq` 特性，所以下面的 if 语句会报错
        enum Foo {
            Bar,
        }

        let a = Foo::Bar;

        // 因为 Foo 类型没有实现 PartialEq 所以会报错
        // if Foo::Bar == a {
        //     // ^-- 编译报错，可以尝试使用 `if let` 替换
        //     println!("a is foobar");
        // }
    }
}

/// Rust 1.65 以后这个特性才规范为正式语法
/// 这个特性实际上就是 if let Some(value) = x { value } else { ! } 的语法糖
/// 这个特性的常用情况是，解构指定的类型出错的情况下需要执行的额外代码，
/// 比如 `break`,`contiune`,`panic!()` 等等情况
fn let_else() {
    {
        use std::str::FromStr;
        fn get_count_item(s: &str) -> (u64, &str) {
            let mut it = s.split(' ');
            // 这里当字符串读取失败了，那就直接 `panic!` 不需要写额外的逻辑了
            let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
                panic!("Can't segment count item pair: '{s}'");
            };
            // 这里同理
            let Ok(count) = u64::from_str(count_str) else {
                panic!("Can't parse integer: '{count_str}'");
            };
            (count, item)
        }

        assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    }

    // 上面的代码使用 `if let` 来写的话是下面这样的，看起来没有上面的更清晰
    {
        use std::str::FromStr;
        fn get_count_item(s: &str) -> (u64, &str) {
            let mut it = s.split(" ");
            let (count_str, item) = match (it.next(), it.next()) {
                (Some(count_str), Some(item)) => (count_str, item),
                _ => panic!("Can't segment count item pair: '{s}'"),
            };
            let count = if let Ok(count) = u64::from_str(count_str) {
                count
            } else {
                panic!("Can't parse integer: '{count_str}'");
            };
            (count, item)
        }

        assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    }
}

fn while_let() {
    {
        // 创建 `Option<i32>` 类型变量
        let mut optional = Some(0);

        // 重复执行 `match`
        loop {
            match optional {
                // 如果 `optional` 可以被 Some 解构则执行代码
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        optional = Some(i + 1);
                    }
                    // ^ 这里需要三级缩进了
                }
                // 如果解构失败则退出循环
                _ => {
                    break;
                } // ^ 这个写法太啰嗦了，应该有更好的写法
            }
        }
    }

    // 为了解决上面的语法啰嗦问题，这里有个简化版的语法糖
    {
        // 创建 `Option<i32>` 类型变量
        let mut optional = Some(0);

        // 重复使用 `Some` 尝试解构 `optional`，
        // 如果解构成功的话就一直执行代码块，否则退出循环
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // 代码清晰了很多，也少了缩进
        }
        // `if let` 语句允许我们使用 `else if`, `else` 来处理更多额外的逻辑
        // `while let` 不支持 `else if`, `else`
    }
}

fn main() {
    // 分支语句
    if_else();

    // 循环语句
    loop_();
    nesting_and_labels();
    returning_from_loops();
    while_();
    for_and_range();
    for_and_iterators();

    // 匹配语句
    match_();
    match_tuples();
    match_array_slice();
    match_enum();
    match_points_or_ref();
    match_struct();
    match_guards();
    match_binding();

    // 判断解构
    if_let();
    let_else();
    while_let();
}
