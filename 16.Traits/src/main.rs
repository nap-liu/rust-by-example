//!
//! 特性(Traits)
//!
//! 特性是一些未知类型的（Self）方法的集合，实现了指定特性就可以访问该特性中定义的其他方法。
//!
//! 特性一个被任何类型实现，下面的代码展示了基础的特性定义和实现。
//!
//!

use std::{
    f64::consts::E,
    fmt::{Debug, Display},
    path::Iter,
};

fn example01() {
    // 定义一个类型
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    // 定义基础的特性函数
    trait Animal {
        // 定义关联函数，通过 `Self` 关键字获取实现该类型的类型声明。
        fn new(name: &'static str) -> Self;

        // 定义关联方法
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // 定义并实现关联方法，在该方法中使用当前特性中定义的其他方法。
        // 这个这个方法有函数体，也就代表这个方法是一个默认方法，实现当前特性的类型可以不实现该方法
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    // 对定义的类型实现一些私有方法。
    impl Sheep {
        // 定义关联方法
        fn is_naked(&self) -> bool {
            self.naked
        }

        // 定义关联方法
        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // 为自定义类型 `Sheep` 实现 自定义特性 `Animal`
    impl Animal for Sheep {
        // 实现特性上的 `new` 方法，这里的 `Self` 指的就是 `Sheep`
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name: name,
                naked: false,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // 特性上的默认实现方法可以重新定义覆盖掉。
        fn talk(&self) {
            // 这里只是作为演示，实际使用的时候可能会做和默认实现完全不同的操作。
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    // 这里必须类型因为 `Animal` 只是一个特性，需要给出明确类型编译器才能确定应该调用哪个实现来返回类型。
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ 尝试移除类型声明 `Sheep`

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

/// 编译器通过 `derive` 属性宏提供了很多基础的特性的快捷实现
///
/// - 对比运算符 `Eq`，`PartialEq`，`Ord`，`PartialOrd`
/// - 克隆运算符（同时复制栈和堆的数据） `Clone`，在把 `&T` 类型转换为 `T` 的时候自动调用
/// - 复制运算符 `Copy` 类型做等号赋值的时候自动调用
/// - 哈希运算符 `Hash` 类型取引用的时候 `&T` 的时候自动调用
/// - 默认值运算符 `Default` 类型的默认值，在数据转移但是转以后原始位置需要防止一个默认值。
/// - 调试运算符 `Debug` 该运算符主要用于格式化语法 `{:?}` 来使用
///
fn derive() {
    // 该类型可以使用对比操作符作对比
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // 该类型可以通过 `{:?}` 格式化
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // 该类型没有任何额外的实现
    struct Seconds(i32);

    let _one_second = Seconds(1);

    // 错误：`Seconds` 没有实现 `Debug` 特性不能被打印
    // println!("One second looks like: {:?}", _one_second);
    // TODO ^ 移除注释查看错误

    // 错误：`Seconds` 因为没有实现 `PartialEq` 特性，所以不能被对比
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ 移除注释查看错误

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}

/// 动态类型
/// 编译器需要明确的知道函数的返回值是什么类型，以及类型需要的内存空间大小是多少，
/// 这样的话所有的函数都必须返回一个固定的实际类型，比如说我们有一个特性是 `Animal`，
/// 但是并不知道实现了该特性的类型是多大，所以没有办法直接使用 `Animal` 来约束函数的返回值。
///
/// 基于上面的问题有简单的解决方案吗？答案是有！
///
/// 我们使用 `Box` 来替换特性的约束，因为 `Box` 是一个实际的类型有固定大小，然后使用 `Box` 来
/// 存放实现了 `Animal` 特性的动态对象。
///
fn returning_traits_with_dyn() {
    // 定义两个类型
    #[derive(Debug)]
    struct Sheep {}
    #[derive(Debug)]
    struct Cow {}

    // 定义特性
    trait Animal {
        fn noise(&self) -> &'static str;
    }

    // 实现特性
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaah!"
        }
    }

    // 实现特性
    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooo!"
        }
    }

    // 使用 `Box<dyn Animal>` 固定返回值需要的尺寸，然后使用 `Box` 来存放一个动态的类型
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

/// 重写操作符
/// `Rust` 提供了很多的特性可以用来重写操作符，很多操作符都可以通过参数来实现不同的逻辑，
/// 可以实现的原因其实就是每一个操作符实际上都是特定函数的语法糖，
/// 比如说 `+` 号操作符实际上就是调用了 `add` 函数，我们可以通过在指定类型上实现 `Add` 特性
/// 实现该特性上定义的 `add` 方法来重新定义类型的加法操作。
fn operator_overloading() {
    // 标准库的操作符导出，这里包含了所有可以重写的操作符。
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // 为 `Foo` 类型实现 `+` 法操作符
    // Add接收一个泛型对象 这个泛型对象会被当做 `add` 函数的第二个参数使用
    // `type Output` 是加法操作的返回值类型
    // 这里实现的效果就是 `Foo + Bar = FooBar`
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");

            FooBar
        }
    }

    // 上面的反向实现 `Bar + Foo = BarFoo`
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");

            BarFoo
        }
    }

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

/// 声明周期结束的时候自动调用 `Drop` 特性的函数。
///
/// 实现了该特性的类型，在实例走出作用域被销毁之前会自动调用该特性的函数，用于做一些自定义的清理操作
///
fn drop_() {
    struct Droppable {
        name: &'static str,
    }

    // `Droppable` 实例被销毁的时候会自动调用该特性的函数
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    let _a = Droppable { name: "a" };

    // 局部块 A
    {
        let _b = Droppable { name: "b" };

        // 局部块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // 手动调用销毁函数
    drop(_a);
    // TODO ^ 尝试注释这行代码

    println!("end of the main function");

    // `_a` 不会因为作用域销毁再次被销毁一次，因为上面已经手动的销毁了一次。
}

/// 迭代器（Iterator）
///
/// 迭代器是一个非常重要的特性，该特性的函数式编程可以让代码更简单易懂，
/// 并且因为不需要额外的安全检查性能会更好！
///
/// 迭代器一般都是使用在类似数组或者集合类的数据上，通过实现迭代器方法可以
/// 让调用方非常轻松的对数据集合进行操作。
///
/// 迭代器特性只需要实现一个方法 `next` 就可以轻松的把集合数据转换成迭代器。
///
///
fn iterators() {
    // 定义一个 `斐波那些数列` 类
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    // 为这个类实现迭代器特性
    impl Iterator for Fibonacci {
        // 迭代器需要提供一个具体的返回值类型，这里的类型为 `u32`。
        type Item = u32;

        // 迭代器要求的返回值必须是 Option 包裹的，当返回为 None 的时候代表迭代器终止，
        // 但是我们这里没有这个情况，因为斐波那些数列是一个无穷大的递增序列。
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;

            // 基于前一个值计算出下一个序列的值
            self.curr = self.next;
            self.next = current + self.next;

            // 因为没有终止的可能所以只需要一直返回结果就可以了
            Some(current)
        }
    }

    // 构造一个实例
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    // 创建一个 `0..3` 区间的迭代器，生成的值为 0,1,2
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` 可以消费迭代器直到迭代器返回 `None` 循环就会终止，循环的变量 `i` 会自动从 `Some` 中解构出数据
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // 迭代器上提供了 `take(n)` 方法，代表从迭代器中取多少个数据，也就是调用多少次 `next` 方法
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // `skip(n)` 方法还可以跳过前面 `n` 个数据，这里就是跳过了前四个数据，然后再获取四个数据，然后终止迭代器。
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // 数组或者切片可以调用 `iter` 方法来创建迭代器。
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

///
/// 函数参数和返回值可以通过 `impl trait` 来约束参数或返回值必须实现某些特性，来实现类似泛型的效果。
///
fn impl_trait() {
    // 通过泛型约束
    {
        fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
            src.lines()
                .map(|line| {
                    // 每一行数据
                    line.map(|line| {
                        // 如果没有发生错误则继续处理
                        line.split(',') // 分割字符串
                            .map(|entry| String::from(entry.trim())) // 移除掉收尾的空字符
                            .collect() // 转换成 Vec<String>
                    })
                })
                .collect() // 转换成 Vec<Vec<String>>
        }
    }

    // 通过特性约束
    {
        fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
            src.lines()
                .map(|line| {
                    // 每一行数据
                    line.map(|line| {
                        // 如果没有发生错误则继续处理
                        line.split(',') // 分割字符串
                            .map(|entry| String::from(entry.trim())) // 移除掉收尾的空字符
                            .collect() // 转换成 Vec<String>
                    })
                })
                .collect() // 转换成 Vec<Vec<String>>
        }
    }

    // 可以看到两种方法都可以实现相同的功能，但是通过特性约束的话会失去手动指定泛型类型的特性，
    // 比如第二个实现就不能通过 `parse_csv_document::<std::io::Empty>(std::io::empty())` 使用。

    {
        use std::iter;
        use std::vec::IntoIter;

        // 把两个 Vec<i32> 类型链接到一起，然后组成一个无限循环的迭代器。
        // 可以看到这个函数的返回值生命非常复杂，但实际上就是一个无限循环的迭代器。
        fn combine_vecs_explicit_return_type(
            v: Vec<i32>,
            u: Vec<i32>,
        ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
            v.into_iter().chain(u.into_iter()).cycle()
        }

        // 通过 `impl Trait` 来约束可以极大的简化返回值类型的声明！！！！
        fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
            v.into_iter().chain(u.into_iter()).cycle()
        }

        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let mut v3 = combine_vecs(v1, v2);
        assert_eq!(Some(1), v3.next());
        assert_eq!(Some(2), v3.next());
        assert_eq!(Some(3), v3.next());
        assert_eq!(Some(4), v3.next());
        assert_eq!(Some(5), v3.next());
        println!("all done");
    }

    // 还有很重要的一点，有些类型不能直接写出来，比如闭包类型，所有的闭包都会生成一个匿名结构，
    // 但是我们无法知道这个匿名类型是什么，所以对于闭包类型我们可以使用 `impl Fn` 来约束闭包的类型。
    {
        // 这里返回了一个闭包，闭包捕获了参数 `y`
        fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
            let closure = move |x: i32| x + y;
            closure
        }
        let plus_one = make_adder_function(1);
        assert_eq!(plus_one(2), 3);
    }

    // 还可以使用 `impl` 来返回一个迭代器的闭包，这样就可以把 `filter` `map` 等操作封装成一个函数。
    {
        fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
            numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
        }

        let singles = vec![-3, -2, 2, 3];
        let doubles = double_positives(&singles);
        assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
    }
}

/// 当把资源赋值给一个变量或者当做参数调用函数的时候都会进行资源的转移，
/// 或者我们需要对资源进行复制，这时候我们就需要使用 `Clone` 特性来支持这个操作了，
/// 大多数情况下我们都可以使用 `.clone()` 方法（该方法是 `Clone` 特性提供的）进行复制数据。
///
fn clone_() {
    // 一个没有数据的结构类型
    #[derive(Debug, Clone, Copy)]
    struct Unit;

    // 元组结构类型
    #[derive(Clone, Debug)]
    struct Pair(Box<i32>, Box<i32>);

    // 实例化 `Unit`
    let unit = Unit;
    // 复制 `Unit` 因为没有数据所以没什么需要复制的
    let copied_unit = unit;

    // 两个 `Unit` 都是独立的
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // 实例化 `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // 移动 `pair` 到 `moved_pair`，这里因为 `pair` 有实际的数据，所以进行了所有权的转移。
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // 错误! `pair` 对数据的所有权已经转移给了 `moved_pair`
    // println!("original: {:?}", pair);
    // TODO ^ 移除注释查看错误

    // 手动调用 `clone` 方法深度复制数据
    let cloned_pair = moved_pair.clone();
    // 销毁原始数据
    drop(moved_pair);

    // 错误! `moved_pair` 已经被销毁了
    //println!("copy: {:?}", moved_pair);
    // TODO ^ 移除注释查看错误

    // `cloned_pair` 还可以使用，因为复制了所有的数据
    println!("clone: {:?}", cloned_pair);
}

fn supertraits() {
    trait Person {
        fn name(&self) -> String;
    }

    // Person 是 Student 的前提条件
    // 当实现 Student 特性的时候也必须同时实现 Person
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    // 同上
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct Engineer {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    impl Person for Engineer {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    impl Student for Engineer {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    impl Programmer for Engineer {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }
    impl CompSciStudent for Engineer {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

    let engineer = Engineer {
        name: String::from("name"),
        university: String::from("university"),
        fav_language: String::from("fav_language"),
        git_username: String::from("git_username"),
    };

    comp_sci_student_greeting(&engineer);
}

/// 因为不管什么类型都可以实现任意数量的特性，这些特性上可能存在相同的名称
/// 这样的话会产生名称冲突，`Rust` 提供了完全限定语法来解决这个问题。
fn disambiguating_overlapping_traits() {
    trait UsernameWidget {
        // 定义方法
        fn get(&self) -> String;
    }

    trait AgeWidget {
        // 定义同名方法
        fn get(&self) -> u8;
    }

    // 基础类型
    struct Form {
        username: String,
        age: u8,
    }

    // 实现特性
    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    // 实现特性
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 错误!：找到了多个同名的方法，因为 `Form` 同时实现了 `AgeWidget` 和 `UsernameWidget` 特性
    // 这两个特性都有一个方法名 `get` 这样的话编译器就没办法知道你想要调用的是哪个方法。
    // println!("{}", form.get());

    // 可以使用完全限定语法来调用指定特性上的方法
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);

    // 还可以使用这个语法
    UsernameWidget::get(&form);
    AgeWidget::get(&form);
}

fn main() {
    // 特性基础展示
    example01();
    // 属性宏快捷实现通用特性
    derive();
    // 使用特性重载操作符
    operator_overloading();
    // 析构函数
    drop_();
    // 迭代器
    iterators();
    // 函数参数和返回值的特性约束
    impl_trait();
    // 数据的复制
    clone_();
    // 特性的继承约束
    supertraits();
    // 特性方法名冲突的解决方案
    disambiguating_overlapping_traits();
}
