//!
//! 泛型（Generics）
//!
//! 泛型是将类型和功能可以用在更多的类型上的一个能力，
//! 它可以通过多种方式非常有效的缩减重复代码，但是带来的问题是会让使用方的语法更复杂。
//!
//! 作为泛型需要非常精确的指定泛型可接受的类型范围。
//! 泛型最简单常用的地方就是当做函数的参数使用了。
//!
//! 泛型函数可以通过 `<T>` (T可以是任意的名称)来声明接收泛型类型，泛型类型需要使用大驼峰命名 `<Aaa, Bbb, ...>`，
//! 如果一个函数声明了泛型参数，并且参数指明使用了泛型中声明的类型，则该函数就是泛型函数，
//! 没有使用泛型声明的参数则是具体类型。
//!
//! 下面这个就是一个泛型的函数
//! ```r
//! fn foo<f>(arg: T) { ... }
//! ```
//! 该函数定义了一个泛型 `T`，声明参数 `arg` 为泛型 `T`。
//! 这个函数可以接受任意类型的参数。
//!

// 一个具体类型 `A`
struct A;

// 定义一个类型 `Single`，该类型拥有一个元组(Tuple)，这个元组里面有一个类型 `A` （这个 `A` 类型就是上面定义的类型）
struct Single(A);
//            ^ 这里定义元组内只包含一个 `A` 类型

// 这里声明了一个泛型 `<T>`，表明 `SingleGen<T>` 类型是一个泛型的类型，
// 然后这个泛型 `T`，放在了元组中。
// 这里泛型 `<T>` 中的 `T` 可以是任意的类型，包含上面定义的 `A`
struct SingleGen<T>(T);

fn main() {
    // `Single` 类型需要一个类型 `A`，因为 `Single` 是具体类型的类型，不是泛型。
    let _s = Single(A);

    // 创建一个变量 `_char` 明确指明类型是 `SingleGen<char>`，
    // 这里通过 `SingleGen('a')` 明确的向泛型传递了一个 `char` 型的参数。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 也可以使用任意的其他类型，泛型可以自动根据参数推断具体的类型。
    let _t = SingleGen(A); // `A` 是上面定义的类型。
    let _i32 = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char` 类型。
}