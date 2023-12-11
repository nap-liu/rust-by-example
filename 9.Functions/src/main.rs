#![feature(never_type)] // 强制启用内置的 `!` 类型

//!
//! # Functions
//! 函数使用 `fn` 关键字定义。必须指定参数类型和数量，
//! 如果函数有返回值的话，必须使用 `->` 把返回值定义在后面
//!
//! 函数的最后一个表达式的返回值会自动当做函数的返回值，
//! 也可以使用 `return` 关键字在代码的任意位置（if\match\loop\等等）让函数提前退出并返回值
//!

fn functions() {
    // 返回 bool 值的函数
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // 边界值直接返回 false
        if rhs == 0 {
            return false;
        }

        // 这个是最后的语句 可以不使用 `return` 关键字，因为语句没有使用 `;` 结尾，
        // 所以函数会使用最后的语句返回值当做整个函数的返回值
        lhs % rhs == 0
    }

    // 这个函数没有返回值，等于返回一个 `()` 空元组（特殊的元组又叫 Unit Value）
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 如果函数的返回值是 `()` 则可以省略返回值的定义
    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }

    fizzbuzz_to(100);
}

/// `关联函数` 和 `关联方法` 这两个概念非常相似
/// `关联函数` 是自定义类型上面和 `实例无关` 的函数定义
/// `关联方法` 是自定义类型上面给 `实例定义` 的操作实例的方法
fn associated_functions_and_methods() {
    struct Point {
        x: f64,
        y: f64,
    }

    // Implementation block, all `Point` associated functions & methods go in here
    // 为 `Point` 类型实现 `关联方法` 和 `关联函数` 的代码块定义
    impl Point {
        // 这个是关联的函数，因为这个函数不需要 `Point` 实例
        // 这个函数只和 `Point` 这个类型有关
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // 另外一个 `关联函数`，这个函数需要两个参数
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // 这个是 `关联方法`，因为函数使用了 `&self` 引用，也就是当前调用函数的实例，
        // 这个是一个语法糖，等价于 `self: &Self`
        // `Self` 是当前的类型的别名也就是 `Rectangle`
        fn area(&self) -> f64 {
            // `self` gives access to the struct fields via the dot operator
            // 使用 `self` 参数通过 `.` 操作符来引用实例上的属性值
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            // `abs` 是 `f64` 类型上的 `关联方法`，所以可以直接通过 `.` 操作符调用
            ((x1 - x2) * (y1 - y2)).abs()
        }

        // 这个也是一个 `关联方法`
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // 这个关联方法，使用了可变引用，`&mut self` 就是 `self: &mut Self` 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // `Pair` 类型拥有两个堆内存上的i32的整数
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // 这个关联方法转移了 `self` 实例的所有权，`self` 就是 `self: Self` 的语法糖
        fn destroy(self) {
            // 解构 `self` 拿到内部属性值
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // 因为 `self` 的所有权转移到了当前作用域，
            // `first` 和 `second` 又把 `self` 上面保存的值转移了出来
            // 当这两个变量走出作用域，那这两个变量的内存就会被回收，同时 `self` 也被回收了
        }
    }

    let rectangle = Rectangle {
        // 关联函数可以使用 `::` 作用域访问操作符来使用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 关联方法可以使用类型实例通过 `.` 操作符来调用，
    // 关联方法的第一个参数会自动使用当前的实例来填充上去
    // `rectangle.perimeter()` 等价于 `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！因为 `rectangle` 是不可变对象，但是 `translate` 方法需要一个可变的对象，所以报错了
    // rectangle.translate(1.0, 0.0);
    // TODO ^ 解除上面这行注释查看错误

    // 这里可以，因为 `square` 是一个可变对象
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // 报错！因为已经调用了 `destroy` 方法，这个方法会转移 `pair` 对象的所有权，
    // 当 `destroy` 方法执行，当前作用域的 `pair` 就已经被转移到了 `destroy` 函数中
    // 当前作用域已经没有 `pair` 对象了，所以会报错
    // pair.destroy();
    // TODO ^ 解除上面这行注释查看错误
}

/// 闭包是一个很重要的概念
/// 闭包和函数非常相似，但是闭包有一个能力就是可以捕获当前上下文的变量，然后延迟执行
/// 这个特性可以让闭包可以动态使用，比常规函数更加灵活
///
/// 闭包的特性和组成部分
/// - 使用 `||` 替换了 `()` 来代替参数的声明
/// - 如果只有一行表达式的话可以省略掉 `{}`
/// - 可以捕获声明的作用域中的变量
///
fn closures() {
    {
        let x = 10;

        let closures = |val| {
            println!("closure capture variable x is: {}", x);
            val + x
        };

        let result = closures(100);

        println!("closures: {}", result);
    }

    {
        let outer_var = 42;

        // 常规的函数不允许引用外部作用域中的变量
        // fn function(i: i32) -> i32 {
        //     i + outer_var
        // }
        // TODO: 移除注释查看错误，编译器会推荐使用闭包

        // 闭包都是匿名的，我们可以把闭包绑定到一个变量上
        // 闭包的变量和返回值声明和函数的规则相同
        // 如果闭包只有一行表达式，那么闭包的 `{}` 是可以省略的
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        let closure_inferred = |i| i + outer_var;

        // 调用闭包和调用常规函数是一样的
        println!("closure_annotated: {}", closure_annotated(1));
        println!("closure_inferred: {}", closure_inferred(1));

        // 如果闭包的类型已经被推断出具体类型了，那么就不允许再次使用其他类型了
        // println!(
        //     "cannot reuse closure_inferred with another type: {}",
        //     closure_inferred(42i64)
        // );
        // TODO: 移除注释查看编译错误

        // 这个闭包没有参数，但是有一个返回值，根据整数推断规则，返回值被推断成 `i32`
        let one = || 1;
        println!("closure returning one: {}", one());
    }
}

/// 闭包的变量捕获
/// 闭包本身是非常灵活的，即便没有类型注释，
/// 也可以根据上下文自动推断出类型，并正常运行
///
/// 闭包可以自动捕获如下类型
/// - &T 变量借用
/// - &mut T 可变变量借用
/// - T 变量移动
fn closures_capturing() {
    {
        use std::mem;
        let mut color = String::from("green");

        // 这个闭包捕获了 `color` 变量，闭包内部使用 `println!` 宏打印变量
        // `println!` 宏只会对使用的变量 `借用一个不变引用` 也就是 `&color`
        // 所以编译器推断 `color` 被闭包借用了一个不变引用，
        // 当这个闭包被最后一次使用后，闭包内借用的不变引用就会被回收

        // 因为 `println!` 只会对使用的变量借用一个不可变引用，所以没有对引用增加更多的检查规则
        let print = || println!("`color`: {}", color);

        // 调用闭包
        // print();

        // 因为 Rust 允许同时存在多个不可变引用，变量 `color` 可以被再次借用。
        let _reborrow = &color;

        // 再次调用闭包，这已经是最后一次使用闭包了，所以这次调用以后，闭包内的引用就会被回收
        print();

        // 因为 `color` 已经不存在其他的引用了，所以可以对变量的所有权进行转移。
        // 你可能好奇，上面明明还有一个借用的变量是 `_reborrow` ，为什么这里可以进行转移？
        // 是因为 `_reborrow` 这个变量没有被使用，在下面这行代码之后 `_reborrow` 也会失效。
        let _color_moved = color;

        let mut count = 0;

        // 因为闭包内对外部变量进行了直接修改，
        // 所以这个闭包需要声明成可变的
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // 调用可变闭包
        inc();

        // 因为闭包还会再次使用，Rust 中当一个可变引用存在的话，就不允许再次使用其他的引用，
        // 所以这里不能再次借用一个不可变的引用
        // let _reborrow = &count;
        // ^ TODO: 移除注释查看错误

        // 再次调用闭包，因为这里是最后一次使用闭包，所以执行完闭包后，闭包就会被回收
        inc();

        // 因为闭包已经被回收了，所以这里可以再次借用新的引用
        let _count_reborrowed = &mut count;

        // `Box` 类型是在堆上储存数据，不像是原始值在赋值的过程中产生拷贝
        let movable = Box::new(3);

        // `mem:drop` 方法是一个泛型方法接收一个 `T` 类型，而且该方法会自动转移变量的所有权
        // 因为变量被闭包捕获的同时，也会对变量的所有权进行转移，一旦闭包被执行过一次
        // 原始的变量就会被移除掉，所以该闭包会被推断成 `FnOnce() ` 类型，并且只能执行一次，
        //
        // 但是如果变量实现了 `Copy` 特性的话，则会隐式的调用 `Copy` 特性来得到一个拷贝，
        // 这样就可以让原始变量可以继续使用，而闭包本身也会自动推断成 `Fn()` 类型
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        // `consume` consumes the variable so this can only be called once.
        // 使用闭包 `consume` 进行变量转移，但是该方法只能调用一次，因为 Box<i32> 没有实现 `Copy`
        consume();
        // consume();
        // ^ TODO: 移除注释查看错误
    }

    {
        // `Vec` 也是一个堆储存的类型
        let haystack = vec![1, 2, 3];

        // 闭包借用了 `haystack` 变量的一个不可变引用，但是因为 `move` 关键字，
        // 会让编译器直接转移变量的所有权，也就是说，闭包直接转移了 `haystack` 变量的所有权，
        // 因为所有权被转移到了闭包内部，所以在后面的代码中都不能使用该变量了
        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // println!("There're {} elements in vec", haystack.len());
        // 移除注释查看错误
        // 上面这行代码会报错，因为闭包直接转移了 `haystack` 变量的所有权

        // 如果移除掉闭包前面的 `move` 关键字，则闭包只会借用一个不可变引用
        // 这样的话上面的代码就不会报错了。
    }
}

/// 闭包当做参数使用
///
/// 在 Rust 中使用闭包特性，大多数情况下都可以让类型推断引擎自动根据上下文推断闭包的类型，
/// 但是当闭包使用在函数的参数的时候，不允许这么做，一旦函数接受一个闭包的话，那就必须要明确指定闭包的类型
///
/// 闭包根据变量的限制规则，有三种类型限制，以下三种限制对于变量的使用限制依次递减
/// - `Fn`: 闭包只能使用不可变引用 `&T`
/// - `FnMut`: 闭包可以使用可变引用 `&mut T`
/// - `FnOnce`: 闭包可以任意使用 `T`，因为直接使用变量会导致变量的所有权转移，所以一旦闭包捕获了变量的所有权，则该闭包只能调用一次
///
/// 编译器会自动检查被捕获变量的使用方式，并选择一个对于变量使用上限制最少的类型
///
/// 假设一个声明为 `FnOnce` 的参数，但实际上闭包可能引用了 `&T`，'&mut T`，`T`,
/// 编译器最终会根据实际闭包中引用的变量类型来决定闭包是什么类型的。
///
/// 因为如果一个变量可以被转移所有权（闭包内直接使用了 `T` 类型，或者闭包前面加上了 `move` 关键字），
/// 那么就代表这个变量可以被借用，但是反过来不行。
///
/// 如果函数参数要求一个 `Fn` 类型的闭包，那么闭包内就不能使用 `&mut T` 和 `T`。
/// 因为 `Fn` 的要求是闭包内只能使用 `&T`。
///
fn as_input_parameters() {
    // 尝试修改函数的闭包约束为 `Fn`，`FnMut`，`FnOnce` 来查看不同的结果

    // <F> 表明 `F` 是一个泛型的的参数，泛型参数由第一个调用方的参数类型来确定具体的类型
    // 函数接受一个闭包，闭包约束为 `FnOnce()` 类型，
    // 该类型的闭包内可以使用 `&T`，`&mut T`，`T`
    fn apply<F>(f: F)
    // where 语句用来约束泛型 `T` 必须是哪些可能的类型
    where
        F: FnOnce(), // 这里约束 F 是一个只能够调用一次，并且没有参数和返回值的函数
    {
        // ^ TODO: 尝试修改类型为 `Fn` 或 `FnMut`。
        f();
    }

    // 这个函数接收一个闭包函数，并返回一个 `i32` 的值。
    fn apply_to_3<F>(f: F) -> i32
    where
        // 闭包参数接受一个 `i32` 的参数，返回一个 `i32` 的值
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;

    let greeting = "hello";
    // 从借用的引用值通过 `to_owned()` 创建一个有所有权的值。
    let mut farewell = "goodbye".to_owned();

    // 闭包会根据借用的变量，以及变量的使用情况来逐步的确认闭包的类型。
    // 该闭包因为借用了一个不可变变量 `&greeting`，和一个转移所有权的变量 `farewell`，
    // 所以该闭包被推断为 `FnOnce` 类型
    let diary = || {
        // 这里借用了一个不可变引用 `&greeting`，不可变引用的闭包类型是 `Fn`，所以这里推断闭包为 `Fn`。
        println!("I said {}.", greeting);

        // 这里又借用了一个可变引用 `&mut farewell`，可变引用的闭包类型是 `FnMut`，所以这里推断为 `FnMut`。
        farewell.push_str("!!!");
        // 因为 `farewell` 已经借用为了 `&mut farewell`，又因为 `&mut farewell` 包含 `&mut` 所以闭包依旧是 `FnMut`。
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 这里因为 `mem::drop` 会要求转移参数 `farewell` 的所有权，所以闭包又被推断为 `FnOnce`。
        mem::drop(farewell);
    };

    // 把闭包通过参数传递给 `apply` 函数，
    // 因为 `apply` 函数要求的正好就是 `FnOnce` 类型的闭包，所以这里可以正常调用
    apply(diary);

    // 这个闭包没有捕获任何的外部变量，所以这个闭包被推断为 `Fn` 类型
    // 闭包接受一个任意类型的参数，并返回参数 * 2的结果。
    // 这里闭包的参数和返回值被推断为 i32 类型的原因是，
    // 我们把闭包传递给了函数 `apply_to_3`，而这个函数的泛型约束为 `Fn(i32) -> i32`，
    // 编译器通过我们的代码行为来自动的为闭包分配了对应的类型。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

/// 闭包可以非常简洁的在声明的上下文作用域中自动捕获一些变量，那么这个简洁的能力带来了哪些问题呢？
/// 当闭包被当做参数传递的时候，函数声明必须是泛型，而且这个泛型必须约束为 `Fn`，`FnMut`，`FnOnce` 的一种
///
/// ```
/// // 这里必须定义成泛型的函数，而且必须对 `f` 参数进行泛型约束
/// fn apply<F>(f: F)
/// where
///     F: FnOnce()
/// {
///     f();
/// }
///
/// ```
///
/// 当闭包被定义的时候，编译器隐式的创建了一个匿名的结构来保存这些被闭包捕获的变量，
/// 这个匿名的结构根据闭包的代码推断出应该实现 `Fn`，`FnMut`，`FnOnce` 的其中一个特性。
/// 变量会自动的绑定到，该匿名的结构体实例，来保留捕获的变量，直到该闭包被调用。
///
/// 匿名的结构体在实例化的时候并不知道具体的类型是什么，
/// 所以类型引擎会通过闭包实例真正被使用的时候，来尝试推断出具体的类型。
///
fn type_anonymity() {
    // 泛型 `F` 是一个闭包，该必须实现 `Fn()` 特性，也就是说
    // 传入的 `f` 参数必须是一个函数，该函数没有参数，并且没有返回值。
    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    let x = 7;

    // 该闭包使用不可变引用捕获了 `x` 变量，并且没有返回值。
    // 编译器会自动生命一个匿名的结构体，该结构体上保存了 `x` 的不可变引用
    // 并且该结构体实现了 `Fn` 的特性。
    let print = || println!("{}", x);

    apply(print);
}

/// 闭包可以当做参数传递给函数，那么常规函数是否也可以呢？
/// 答案是可以，只要函数的实现可以满足泛型 `Fn`，`FnMut`，`FnOnce` 的约束即可
fn input_functions() {
    // 定义一个泛型参数约束为一个闭包类型，该闭包可以引用外部的不可变变量
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // 定义一个可以符合 `Fn` 特性约束的普通函数。
    fn function() {
        println!("I'm a function!");
    }

    // 定义一个符合 `Fn` 特性约束的闭包。
    let closure = || println!("I'm a closure!");

    // 传递闭包
    call_me(closure);
    // 传递普通函数
    call_me(function);
}

/// 闭包可以当做函数参数传递，那么闭包也可以当做函数返回值传递
/// 因为匿名的闭包类型是由编译器自动定义的匿名类型，所以我们不可能提前知道匿名的类型是什么，
/// 但是我们可以知道匿名函数实现了什么特性比如说 `Fn`，`FnMut`，`FnOnce`，
/// 所以我们可以通过使用 `impl` 关键字来约束返回值的类型，来间接的使用闭包返回值。
fn as_output_parameters() {
    // 这里使用 `impl` 关键字约束返回值类型是 `Fn`
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        // 这里必须要使用 `move` 关键字，因为一旦当前的函数结束以后
        // 这个函数中的所有变量都会被回收，因为闭包中捕获了变量 `text`，
        // 所以我们需要让这个 `text` 强制转移到闭包的内部，来避免当前函数结束后 `text` 被回收的情况。
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_plain();

    fn_mut();
    fn_mut();

    fn_once();

    // 下面这行代码会报错，因为 `fn_once` 的约束是 `FnOnce` 所以只能调用一次
    // fn_once();
    // 移除注释查看错误
}

/// 标准库 `Iterator::any`
/// 该函数会把迭代器中的所有元素都传递给闭包，闭包来返回一个 `bool` 值，
/// 如果有任意一个闭包的返回值是 `true`，则 `Iterator::any` 方法会返回 `true`，否则返回 `false`
///
/// 该方法声明如下
///
/// ```
/// pub trait Iterator {
///     // 传递进来的闭包函数接受的参数类型
///     type Item;
///     
///     // `any` 接受一个 `&mut self` 的可变引用，
///     // 也就是说当前方法可以修改数据，但是只是修改而不是转移所有权
///     fn any<F>(&mut self, f: F) -> bool
///     where
///         // 传递的闭包函数接受一个参数，并且该闭包可以对参数进行修改
///         F: FnMut(Self::Item) -> bool;
/// }
/// ```
///
///
fn iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` 会生成一个对原始数据引用的一个迭代器，然后调用 `any` 方法传递了一个闭包
    // 因为 `any` 方法回调的时候会传递原始数据的引用 `&T`，
    // 根据结构规则，`&T` 和 `&x` 会同时去掉 `&` 所以，闭包内部的 `x` 是 `i32` 类型
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // `into_iter()` 方法会直接转移 `vec2` 的所有权并把原始数据转换成迭代器，
    // 所以这里 `any` 的闭包中不需要使用解构语法就可以拿到 `i32` 类型
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // 因为 `iter()` 只是借用了 `vec1` 中的值，所以这里可以再次使用 `vec1`。
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    // `into_iter()` 因为转移了 `vec2` 的所有权，所以这里不能再次使用 `vec2` 了
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: 移除注释查看错误。

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` 方法同样可以在数组中使用，这里依旧是借用了数据，所以需要解构。
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` 方法转移了数组的所有权，所以这里不需要解构，但是后面就不能再次使用原数组了。
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}

/// 标准库 `Iterator::find` 方法
/// 该方法接受一个 `&mut self` 引用，和一个闭包函数，
/// 当闭包返回 `true` 的时候，该函数会返回对应的传递给闭包的那一项数据。
/// 闭包函数接收一个 `&Self::Item` 不可变引用，并返回一个 `bool` 值，
/// 当闭包返回 `true` 整个函数就会退出并返回值。
///
/// `Iterator::find` 定义如下
/// ```
/// pub trait Iterator {
///     // 闭包要接收的参数类型。
///     type Item;
///
///     // `find` 接受一个 `&mut self` 参数，通过 `Option` 来返回结果。
///     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
///         // 闭包接受一个 `&Self::item` 的引用参数，并返回一个 `bool` 值。
///         P: FnMut(&Self::Item) -> bool;
/// }
/// ````
fn searching_through_iterators() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` 创建一个不可变引用的迭代器，该迭代器中的数据类型是 `&i32`。
    let mut iter = vec1.iter();
    // `into_iter()` 把 `vec2` 转移到新创建的迭代器中，该迭代器的数据类型是 `i32`。
    let mut into_iter = vec2.into_iter();

    // 因为 `iter()` 创建出的迭代器类型是 `&i32` 类型，而根据 `find` 函数定义，
    // 会再次给 `&i32` 类型再次创建一个不可变引用，也就是 `&(&i32)` == `&&i32`，
    // 然后根据解构规则，这里的闭包需要两个解构符号 `&&x` 来获取 `i32` 类型。
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));

    // 因为 `into_iter()` 创建出的迭代器类型是 `i32` 类型，而根据 `find` 函数定义，
    // 会给 `i32` 类型创建一个不可变引用，也就是 `&i32`，
    // 然后根据解构规则，这里的闭包需要解构 `&x` 来获取 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` 数组上也是一样的规则，`iter()` 转换成 `&i32`，
    // `find` 会再次添加一个引用变成 `&&i32`，
    // 所以闭包中解构需要 `&&x`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));

    // `into_iter()` 转移了数组的所有权到迭代器中，类型为 `i32`，
    // `find` 会添加一个引用变成 `&i32`，
    // 所以闭包中解构需要 `&x`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}

/// 高阶函数
/// Rust 中为迭代器(Iterator)，提供了很多高阶的函数式函数，
/// 因为迭代器本身的实现是惰性的，而且因为迭代器可以避免数组、切片的越界检查，
/// 因此迭代器的性能会比普通的循环语句效率要高的，因为通过 `[]` 访问数组或切片都会进行运行时的检查，会消耗更多的性能。
///
/// 所以在 Rust 中应该优先使用迭代器，而且 Rust 提供了很多内置的函数式方法来让代码逻辑更清晰。
///
/// 下面是同一个功能的两种代码实现。
///
fn higher_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let upper = 1000;
    println!(
        "Find the sum of all the squared odd numbers under {}",
        upper
    );

    // 传统 for 循环写法
    {
        // 定义一个计数器变量
        let mut acc = 0;
        // 声明一个无限长度的迭代器
        for n in 0.. {
            // 把数字进行平方计算
            let n_squared = n * n;

            if n_squared >= upper {
                // 如果当前的平方结果大于上限的话则跳出循环
                break;
            } else if is_odd(n_squared) {
                // 如果平方结果是奇数的话，计数器加一
                acc += n_squared;
            }
        }
        println!("imperative style: {}", acc);
    }

    // 迭代器函数式写法
    {
        let sum_of_squared_odd_numbers: u32 = (0..) // 无限长度的迭代器
            .map(|n| n * n) // 对数字取平方
            .take_while(|&n_squared| n_squared < upper) // 平方结果到达上限的话 终止迭代器
            .filter(|&n_squared| is_odd(n_squared)) // 把平方的奇数保留下来
            .sum(); // 统计所有符合条件的数字数量
        println!("functional style: {}", sum_of_squared_odd_numbers);
    }
}

/// 分流函数是一个特殊的函数，该函数的返回值定义是 `!`，
/// `!` 关键字表示是一个空的类型(never\never_type)，也就是说函数永远不会返回。
///
/// ```
/// // 该函数永远没有返回值，因为 `panic!` 宏会直接终止进程！！
/// fn foo() -> ! {
///     panic!("This call never returns.")
/// }
/// ```
/// `!` 和 `()` 不同，`!` 的所有可能的值都不存在，而 `()` 有一个可能的值是 `()` 本身。
///
///
///
fn diverging_functions() {
    // 返回值是 `()` 类型
    fn some_fn() -> () {
        () // 返回一个 `()` 的实例
    }

    // `_a` 的类型是 `()`
    let _a: () = some_fn();

    println!("This function returns and you can see this line.");

    // `panic!` 宏不会返回，也没有返回值。
    // let x: ! = panic!("This call never returns.");
    // 因为 `panic!` 宏不会返回，所以这行代码永远不会执行。
    // println!("You will never see this line!");
    // 尝试移除注释查看结果

    // `!` (never_type) 的另一个非常有用的场景就是 `match` 模式匹配中的流程控制语句 `continue`，
    // `contiune` 语句会强制跳转代码到下一次迭代的起始点，也就是说 `continue` 语句的返回值就是 `!`
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 这里 `match` 语句必须返回 `u32` 类型的数据。
            let addition: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                // 这里的 `i` 是 `u32` 类型，所以可以直接使用。
                true => i,
                // 这里使用了 `contiune` 关键字，而这个关键字会改变代码的执行流程，
                // 因为改变了代码的执行流程，所以 `contiune` 永远都不会返回，也就不会违反类型检查规则。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );

    // 还有比如说 `loop` 关键字也不会返回值，还有 `exit` 方法不会返回值，或者说一个无限循环的函数也不会返回值，等等的一些场景。
}

fn main() {
    // 常规函数
    functions();

    // 关联函数、关联方法
    associated_functions_and_methods();

    // 基础闭包
    closures();
    // 闭包捕获以及闭包的类型推断
    closures_capturing();
    // 闭包捕获的原理
    type_anonymity();
    // 普通函数当做参数传递
    input_functions();
    // 闭包当做返回值使用
    as_output_parameters();

    // 标准库的一些闭包例子
    iterator_any();
    searching_through_iterators();

    // 高阶函数
    higher_order_functions();

    // 分流函数 never_type
    diverging_functions();
}
