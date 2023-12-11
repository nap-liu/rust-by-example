//!
//! 泛型实现（implementation）
//!
//! 和函数类似，实现也可以使用泛型
//!
//! ```
//! struct S; // 具体类型 `S`
//! struct GenericVal<T>(T); // 泛型 `GenericVal`
//
//! // 使用具体类型实现 GenericVal 类型
//! impl GenericVal<f32> {} // 对于 `f32` 类型的实现
//! impl GenericVal<S> {} // 对于 `S` 类型的实现
//!
//! // `<T>` 继续保持示范性，也就是任意类型的实现
//! impl<T> GenericVal<T> {}
//! ```
//!
struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// 具体类型的实现
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// 泛型类型对于任意的类型的实现
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
