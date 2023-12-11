//!
//! 泛型特性 (trait)
//!
//! 特性也可以是泛型的，下面是一个泛型特性的示例。
//!

struct Empty;
struct Null;

// 定义一个泛型的特性
trait DoubleDrop<T> {
    // 函数会转移实例的所有权，并且接收的参数也会转移所有权，
    // 当函数执行以后，会自动移除实例和传入的参数
    fn double_drop(self, _: T);
}

// 声明两个泛型类型 `T`、`U`，然后为泛型 `U` 实现上面定义的泛型 `DoubleDrop` 特性
// 因为泛型 `T` 和 `U` 都没有明确的类型也就是可以是任意的类型，也就是说这个实现会对所有的类型都生效！！！
impl<T, U> DoubleDrop<T> for U {
    // 该函数转移实例和参数的所有权，然后直接丢弃，这样就实现了 `drop` 的功能。
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // 丢弃了 `empty` 和 `null` 两个实例。
    empty.double_drop(null);

    // empty;
    // null;
    // ^ TODO: 尝试移除注释
}
