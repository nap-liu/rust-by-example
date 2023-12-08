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
        let color = String::from("green");

        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last time.
        //
        // `println!` only requires arguments by immutable reference so it doesn't
        // impose anything more restrictive.
        let print = || println!("`color`: {}", color);

        // Call the closure using the borrow.
        print();

        // `color` can be borrowed immutably again, because the closure only holds
        // an immutable reference to `color`.
        let _reborrow = &color;
        print();

        // A move or reborrow is allowed after the final use of `print`
        let _color_moved = color;

        let mut count = 0;
        // A closure to increment `count` could take either `&mut count` or `count`
        // but `&mut count` is less restrictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
        // calling the closure mutates the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // Call the closure using a mutable borrow.
        inc();

        // The closure still mutably borrows `count` because it is called later.
        // An attempt to reborrow will lead to an error.
        // let _reborrow = &count;
        // ^ TODO: try uncommenting this line.
        inc();

        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // possible to reborrow without an error
        let _count_reborrowed = &mut count;

        // A non-copy type.
        let movable = Box::new(3);

        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        // `consume` consumes the variable so this can only be called once.
        consume();
        // consume();
        // ^ TODO: Try uncommenting this line.
    }

    {
        // `Vec` has non-copy semantics.
        let haystack = vec![1, 2, 3];

        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // println!("There're {} elements in vec", haystack.len());
        // ^ Uncommenting above line will result in compile-time error
        // because borrow checker doesn't allow re-using variable after it
        // has been moved.

        // Removing `move` from closure's signature will cause closure
        // to borrow _haystack_ variable immutably, hence _haystack_ is still
        // available and uncommenting above line will not cause an error.
    }
}

fn main() {
    // 常规函数
    functions();

    // 关联函数、关联方法
    associated_functions_and_methods();

    // 基础闭包
    closures();
    // 闭包捕获
    closures_capturing();
}
