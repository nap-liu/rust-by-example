//!
//! 声明周期 (Lifetimes)
//!
//! 生命周期的作用是为了让编译器可以明确的知道每一个变量从什么时候开始到什么时候结束。
//! 声明周期和作用域通常都是一起出现的，但是这两个是不同的概念。
//!
//! 声明周期取决于当前变量声明的作用域，当变量发生转移则作用域也会转移。
//!
extern crate rand;

/// 下面每个变量的线条指示了该变量的生命周期
fn example01() {
    let i = 3; // 变量 `i` 的生命周期开始           ────────────────┐
               //                                                     │
    {
        //                                                            │
        let borrow1 = &i; // `borrow1` 生命周期开始       ───────┐│
                          //                                         ││
        println!("borrow1: {}", borrow1); //                         ││
    } // 变量 `borrow1` 的生命周期终止   ───────────────────────────────┘│
      //                                                              │
      //                                                              │
    {
        //                                                            │
        let borrow2 = &i; // `borrow2` 生命周期开始       ───────┐│
                          //                                         ││
        println!("borrow2: {}", borrow2); //                         ││
    } // 变量 `borrow2` 的生命周期终止 ─────────────────────────────────┘│
      //                                                              │
} // 生命周期结束     ──────────────────────────────────────────────────┘

/// 明确声明生命周期
/// 可以使用 `'a` 关键字其中 `a` 可以是任意的字母组合，通常来说都是小写的字母
/// 比如 `foo<'a>` 表示 `foo` 有一个生命周期参数是 `'a`
fn explicit() {
    // `print_refs` 有两个生命周期参数，这两个参数 `x` 和 `y` 都有各自独立的生命周期 `'a` 和 `'b`。
    // 这两个参数的生命周期必须大于等于当前函数的生命周期。
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    // 这个函数有一个生命周期参数 `'a` 但是该声明周期并没有被参数使用，
    // 所以这里的 `'a` 等于 `'static` 也就是整个应用的生命周期
    fn failed_borrow<'a>() {
        let _x = 12;

        // 错误： `_x` 的生命周期短于参数的生命周期 `'a`
        // let y: &'a i32 = &_x;
        // 因为 `'a` 没有被参数使用，所以会被推断为 `'static`，又因为
        // `_x` 的生命周期仅仅存在于函数中，根据生命周期规则，一个小的生命周期不能赋值给大的生命周期。
    }

    // 创建两个变量用于下面的函数。
    let (four, nine) = (4, 9);

    // 传递两个引用到函数中。
    print_refs(&four, &nine);
    // 出借方一定要比借用方存活的时间更长。
    // 也就是说这里 `&four` 和 `&nine` 要比函数 `print_refs` 的生命周期更长，
    // 因为函数调用完以后函数就不再需要了，但是变量还可以继续使用。

    failed_borrow();
    // 由于函数 `failed_borrow` 没有参数，所以没有任何方式来约束 `'a` 和 `failed_borrow` 的生命周期关系，
    // 所以编译器会把 `'a` 推断为 `'static`，这就代表 函数中所有使用 `'a` 来声明的变量都可以在函数外的任意位置使用，
    // 也就是整个应用只要存活就可以使用。
}

/// 这里不谈[省略语法](https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html)，函数的签名有两个强制约束，
/// - 任何的引用值必须有生命周期标注。
/// - 任何的返回值引用必须引用 输入的参数生命周期或者明确指定为 `'static`
///
/// 如果返回的引用值和输入没有关联的话则会导致对数据的无效引用（特殊 `'static` 标注不受此规则限制），所以必须禁止！！
///
fn functions() {
    // 函数定义了一个 `'a` 生命周期，这个周期被参数 `x: &'a i32` 引用，也就是 `x` 的生命周期必须大于函数。
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    // 同样的也可以使用可变引用 `mut` 关键字
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    // 多个参数声明不同的生命周期，也可以多个参数使用同一个生命周期。
    // 不同生命周期的话代表两个参数可以使用不同生命周期的变量。
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    // 声明两个生命周期，返回值和第一个声明周期关联，所以这个函数返回值只能是声明周期为 `'a` 的 `x` 参数。
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    // 这里因为函数没有参数，所以生命周期会自动推断成 `'static`，因为字符串（"foo"）和 `String` 的生命周期为当前函数作用域，
    // 当函数结束后作用域销毁，导致字符串("foo") 和 `String` 被销毁，返回的引用就变成了无效的引用，所以报错。
    // fn invalid_output<'a>() -> &'a String {
    //     &String::from("foo")
    // }

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

/// 方法和函数的生命周期规则是一样的参考函数即可
fn methods() {
    struct Owner(i32);

    impl Owner {
        // 声明声明周期为实例的生命周期
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'a>(&'a self) {
            println!("`print`: {}", self.0);
        }
    }

    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

fn structs() {
    // 结构体声明一个生命周期和构建的实例参数声明周期相同
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // 结构体中的生命周期和构建的生命周期相同，而且每个字段的生命周期也相同。
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // 枚举中也可以指定生命周期，还可以指定部分的特定值的生命周期。
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

/// 特性也支持定义生命周期，规则和函数非常相似
/// 只不过 `impl` 关键字可能拥有很多个声明周期声明。
fn traits() {
    // 声明一个有生命周期的结构体
    #[derive(Debug)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    // 实现标准库的 `Default` 特性并指明声明周期
    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            // 这里直接使用字面量，让编译器把字面量的生命周期声明成 `'a`
            Self { x: &10 }
        }
    }

    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}

/// 泛型约束中的生命周期
fn bounds() {
    use std::fmt::Debug; // 引用标准库

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);
    // `Ref` 结构体拥有一个泛型参数和一个未知的生命周期 `'a` ，并且约束了 `T` 类型的声明周期为 `'a`，
    // 也就是说泛型 `T` 必须要能存活大于等于 `'a` 才可以，
    // 换句话说就是 构 `Ref` 的实例的的参数必须要大于 `Ref` 实例的生命周期。

    // 泛型参数 `T` 使用 `where` 关键字约束为必须实现 `Debug` 特性。
    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("`print`: t is {:?}", t);
    }

    // 函数 `print_ref` 有两个泛型参数 泛型 `T` 和一个未知的生命周期 `'a`，
    // 参数 `t` 被类型被定义为泛型 `T` 并且类型 `T` 的生命周期和定义的 `'a` 绑定，
    // `where` 语句约束了泛型 `T` 必须实现 `Debug` 特性并且要满足声明周期 `'a`，
    // 也就是说参数 `t` 可以是任意实现了 `Debug` 特性的类型，但是参数的生命周期必须大于函数运行的生命周期。
    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("`print_ref`: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

/// 强制缩短声明周期
/// 当生命周期不同的时候需要手动指定声明周期，编译器会尝试自动强制让生命周期相同，
/// 或者手动强制约束多个生命周期之间的关系。
fn coercion() {
    // 编译器会自动推断并对齐两个参数之间的生命周期
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    // 这里 `<'a: 'b, 'b>` 强制让 'a 生命周期必须大于等于 'b 的声明周期。
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    let first = 2; // 长一点的生命周期

    {
        let second = 3; // 短一点的生命周期

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}

/// 静态声明周期
///
/// Rust 中有几个特殊的生命周期，其中有一种是 `'static`，
/// 有两种使用场景，
///
/// ```
/// // 声明静态字符串
/// let s: &'static str = "hello world";
/// // 泛型的静态约束
/// fn generic<T>(x: T) where T: 'static {}
/// ```
///
fn statics() {
    {
        // 声明 `NUM` 是静态生命周期的变量。
        static NUM: i32 = 18;

        // 固定返回静态的变量 `NUM`，这里发生了强制转换，因为 `'static` 的存活时间最久，所以允许转换。
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        {
            // 一个字面量的字符串 `static_string`，字面量天生用于 `'static` 生命周期，
            // 尽管当前的作用域被销毁了，引用也被回收了，但是这个字符串依旧存在于二进制文件中不会被销毁。
            let static_string = "I'm in read-only memory";
            println!("static_string: {}", static_string);
        }

        {
            // 定义一个字面量 `lifetime_num`
            let lifetime_num = 9;

            // 使用这个字面量变量调用 `coerce_static` 返回固定的 `NUM` 引用。
            let coerced_static = coerce_static(&lifetime_num);

            println!("coerced_static: {}", coerced_static);
        }

        println!("NUM: {} stays accessible!", NUM);
    }

    // 使用 `Box::leak` 动态创建 `'static` 变量。
    // `'static` 声明周期实际上只是要求在该变量存在以后可以一直使用，所以可以在程序运行中
    // 动态的创建 `'static` 的引用值，这里可以通过标准库提供的 `Box::leak` 方式来动态创建一个 `'static` 的引用。
    {
        use rand::Fill;

        fn random_vec() -> &'static [usize; 100] {
            let mut rng = rand::thread_rng();
            let mut boxed = Box::new([0; 100]);
            boxed.try_fill(&mut rng).unwrap();
            Box::leak(boxed)
        }

        let first: &'static [usize; 100] = random_vec();
        let second: &'static [usize; 100] = random_vec();
        assert_ne!(first, second)
    }

    // 使用 特性 + 'static 来约束参数
    {
        use std::fmt::Debug;

        fn print_it(input: impl Debug + 'static) {
            println!("'static value passed in is: {:?}", input);
        }

        // 字面量天生拥有 `'static` 所以可以正常调用
        let i = 5;
        print_it(i);

        // 这里对 `i` 借用了一个不可变引用，但是该引用只存在于当前作用域，所以不满足 `'static` 约束
        // print_it(&i);
        // TODO 移除注释查看错误
    }
}

/// 声明周期的省略写法
///
/// 编译器可以允许省略一些常见的生命周期写法，这样可以提高代码的可读性，
/// 详细的可省略的写法可以[参考这里](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)
///
fn elision() {
    // 当只有一个参数的时候可以省略生命周期 `elided_input` 和 `annotated_input` 是一样的。
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }

    // 如果只有一个参数和返回值的话也可以省略生命周期，`elided_pass` 和 `annotated_pass` 也是一样的。
    // 如果只有一个参数和返回值 那么参数和返回值会自动的分配声明周期为 `'a` 也就是 `elided_pass` 会
    // 自动补全成 `annotated_pass` 的写法。
    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

fn main() {
    // 生命周期的基础展示
    example01();
    // 明确指定声明周期
    explicit();
    // 函数中的生命周期关系
    functions();
    // 方法中的生命周期关系
    methods();
    // 结构体中的生命周期关系
    structs();
    // 特性中的生命周期
    traits();
    // 泛型约束中的生命周期
    bounds();
    // 声明周期的强制转换
    coercion();
    // 字面量和 `'static`
    statics();

    println!("Hello, world!");
}
