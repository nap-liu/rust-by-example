//! 宏 (macro_rules!)
//!
//! 宏是一个非常强大的元编程系统，前面已经见过了类似于函数的宏，宏使用 `!` 结尾的函数，
//! 宏不像是代码能生成实际的内容，宏更像是一个预处理器，在代码被编译之前先对代码进行加工修改，
//! 然后再进行编译，在 Rust 中宏并不是对源码的处理，而是对源码的抽象语法树（AST）进行处理，
//! 这样能避免因为优先级出现的问题。
//!

///
/// 这是一个最基础的宏示例
///
/// 为什么宏非常有用？
///
/// - 不需要写重复代码，可能很多情况下都会需要有大块相同但是又有细微不同的重复代码，这时候就可以用宏来优化代码结构。
/// - DSL，自定义特定的语法场景。
/// - 可变参数，有很多场景需要参数能根据数量动态变化，这样的好处是函数非常通用，可以让 `API` 使用起来更方便。
///
fn example01() {
    // 使用 `macro_rules!` 关键字定义一个 `say_hello` 宏
    macro_rules! say_hello {
        // `()` 表示没有参数
        () => {
            // 这里的代码都会展开到调用处
            println!("Hello!")
        };
    }

    // 调用自定义宏
    say_hello!();
}

/// 宏参数以及基础使用
///
/// 宏的参数提供了多种格式
///
/// - block
/// - expr 表达式
/// - ident 标识符
/// - item
/// - literal 字面量
/// - pat match 的子语句
/// - path
/// - stmt 语句块
/// - tt (token tree) 操作符
/// - ty (类型)
/// - vis (可见性限定符)
///
/// 完整列表可以[查看这里](https://doc.rust-lang.org/reference/macros-by-example.html)
///
fn designators() {
    macro_rules! create_function {
        // 宏接收一个参数，这个参数是一个标识符（ident），
        // 宏展在使用处展开后会使用参数的标识符生成一个函数。
        ($func_name: ident) => {
            fn $func_name() {
                // 这里使用 `stringify!` 宏把一个标识符转换成字符串形式；
                println!("You called {:?}", stringify!($func_name));
            }
        };
    }

    macro_rules! print_result {
        // 这个宏接收一个表达式为参数，当宏被展开的时候，表达式会被替换成下面的代码。
        // 这里依旧使用 `stringify!` 宏把收到的表达式转换为了字符串的字面量放到了输出的代码中。
        ($expression: expr) => {
            println!("{:?} == {:?}", stringify!($expression), $expression);
        };
    }

    // 通过宏定义了两个函数
    create_function!(foo);
    create_function!(bar);

    // 调用宏生成的函数
    foo();
    bar();

    // 调用宏，把表达式转换成宏生成的代码。
    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

/// 宏的重载
fn overload() {
    // 这个宏可以接收两种不同的参数格式
    macro_rules! test {
        // 第一种宏格式 `test!(a; and b)`
        ($left: expr; and $right: expr) => {
            println!(
                "{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right
            )
        };
        // 第二种宏格式 `test!(a; or b)`
        ($left: expr; or $right: expr) => {
            println!(
                "{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right
            )
        };
    }

    test!(1i32 + 1== 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

fn repeat() {
    // 这个宏会递归展开参数直到全部展开
    macro_rules! find_min {
        // 当参数只有一个的时候那就是最小值
        ($x: expr) => {
            $x
        };
        // 当参数大于一个的时候需要递归展开
        // 这里相当于 `$x` 是第一个参数，`$($y: expr),+` 是后面的所有参数。
        // 实际上这里的 `$(...),+` 类似于正则表达式 `$(...),` 是要尝试重复匹配的模式 `+` 号代表 1到无数次。
        ($x: expr, $($y: expr),+) => {
            // 然后这里使用的时候类似于 `$($y),+` 是剩余的列表不断地递归展开剩余的参数。
            std::cmp::min($x, find_min!($($y),+))
        };
    }

    // 这里 `find_min!(1)` = `1`
    println!("{}", find_min!(1));
    // 这里 `find_min!(1 + 2, 2)` = `std::cmp::min(1 + 2, 2)`
    println!("{}", find_min!(1 + 2, 2));
    // 这里 `find_min!(5, 2 * 3, 4)` = `std::cmp::min(5, std::cmp::min(2 * 3, 4))`
    println!("{}", find_min!(5, 2 * 3, 4));
}

/// 使用自定义宏来生成重复的代码，
/// 还可以使用宏定义还生成重复的测试用例
fn dont_repeat_yourself() {
    pub mod macro_test {
        use std::ops::{Add, Mul, Sub};

        macro_rules! assert_equal_len {
            // 使用 `$op:tt` 类型来捕获操作符
            ($a:expr, $b:expr, $func:ident, $op:tt) => {
                assert!(
                    $a.len() == $b.len(),
                    "{:?}: dimension mismatch: {:?} {:?} {:?}",
                    stringify!($func),
                    ($a.len(),),
                    stringify!($op),
                    ($b.len(),)
                );
            };
        }

        macro_rules! op {
            ($func:ident, $bound:ident, $op:tt, $method:ident) => {
                // 定义宏展开的函数模版
                fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
                    assert_equal_len!(xs, ys, $func, $op);

                    for (x, y) in xs.iter_mut().zip(ys.iter()) {
                        *x = $bound::$method(*x, *y);
                    }
                }
            };
        }

        // 使用宏快速定义三个方法
        op!(add_assign, Add, +=, add);
        op!(mul_assign, Mul, *=, mul);
        op!(sub_assign, Sub, -=, sub);

        pub mod tests {
            use std::iter;
            macro_rules! test {
                ($func:ident, $x:expr, $y:expr, $z:expr) => {
                    fn $func() {
                        for size in 0usize..10 {
                            let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                            let y: Vec<_> = iter::repeat($y).take(size).collect();
                            let z: Vec<_> = iter::repeat($z).take(size).collect();

                            super::$func(&mut x, &y);

                            assert_eq!(x, z);
                        }
                    }
                };
            }

            // 使用宏来生成三种测试用例函数。
            test!(add_assign, 1u32, 2u32, 3u32);
            test!(mul_assign, 2u32, 3u32, 6u32);
            test!(sub_assign, 3u32, 2u32, 1u32);

            pub fn start_tests() {
                // 这里调用的是当前的模块内宏展开后的函数
                add_assign();
                mul_assign();
                sub_assign();
            }
        }
    }

    // 调用测试用例
    macro_test::tests::start_tests();
}

/// 自定义语法 （DSL）
///
/// `Rust` 中可以使用宏来实现自定义语法的扩展，
/// 因为宏会先进行展开变成合法的 `Rust` 代码
///
fn domain_specific_languages() {
    macro_rules! calculate {
        // 这里使用双花括号来定义宏展开后的内容，因为宏是使用 `calculate! { eval 1 + 2 }` 来使用的。
        // 自定义了一个关键字叫 `eval` 这个关键字后面接收一个语句。
        (eval $e:expr) => {
            {
                let val: usize = $e; // 强制转换捕获到的表达式结果为数字
                println!("{} = {}", stringify!{$e}, val);
            }
        };
    }

    calculate! {
        eval 1 + 2 // `eval` 是自己定义的关键字
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}

/// 多态，可变参数宏
fn variadic_interfaces() {
    macro_rules! calculate {
        // 一个参数的情况直接进行展开。
        (eval $e: expr) => {{
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }};
        // 不定参数情况下动态展开
        (eval $e: expr, $(eval $r: expr),+) => {{
            // 独立的一个参数进行直接展开
            calculate! { eval $e }
            // 剩余的参数继续递归展开
            calculate! { $(eval $r),+ }
        }}
    }

    calculate! { // 可变多态宏 `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

fn main() {
    // 宏的基本使用
    example01();
    // 宏的参数
    designators();
    // 宏的多态
    overload();
    // 可变参数宏
    repeat();
    // 定义宏函数模版
    dont_repeat_yourself();
    // 自定义语法 DSL
    domain_specific_languages();
    // 可变多态宏
    variadic_interfaces();
}
