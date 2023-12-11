//!
//! where 子语句
//! 在声明泛型类型的时候可以通过 `where` 子语句把泛型约束从签名中提取出来，
//! 这样可以让声明看起来更简洁一些，而且 `where` 子语句可以限定任意的类型，而不仅仅是函数参数。
//!
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里使用 `where` 子语句重新定义了 `T` 类型为 `Option<T>`。
//
impl<T> PrintInOption for T
where
    Option<T>: Debug, // 这里使用了类型的重定义，把 `T` 重新定义成 `Option<T>` 然后对对重定义的类型进行约束
{
    // 这里的 `self` 就是 `T` 的实例，然后使用 `Some` 包装 `self` 就可以打印了，
    // 因为 `Some` 实现了 `Debug` 特性。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
