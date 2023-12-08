fn main() {
    // 明确声明变量类型
    let logical: bool = true;

    let a_float: f64 = 1.0; // 常规类型声明
    let an_integer = 5i32; // 后缀类型声明

    // 默认的类型推断，整数型默认 i32，浮点型默认是 f64
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // 变量类型还可以根据代码的上下文自动推断
    let mut inferred_type = 12; // 这里的类型推断为 i64 是因为下面一行代码写的数值范围超过了 i32 类型的有效范围
    inferred_type = 4294967296i64; // 这里的数值超过了默认 i32 类型的取值范围，所以进行了类型扩容推断为 i64

    // 所有的变量默认都是不可变类型，通过添加 mut 关键字可以让变量的值可修改
    let mut mutable = 12; // 可变的 `i32` 类型
    mutable = 21;

    // 错误！变量可变的前提条件是，变更的值类型不能变，这里报错就是因为尝试把 i32 类型的变量修改为 bool 类型
    mutable = true;

    // 变量可以通过 let 关键字重新定义（遮蔽：shadowing）来修改变量的类型。
    let mutable = true;
}
