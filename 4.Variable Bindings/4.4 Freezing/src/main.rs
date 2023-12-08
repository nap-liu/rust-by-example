//!
//! 可以通过使用子作用域遮蔽（shadowing）父作用域的同名可变变量成不可变变量
//! 这样在这个子作用域内的同名变量就会是 冻结（Freezing）状态
//! 直到子作用域结束
//!
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 使用不可变遮蔽父作用域的同名变量
        let _mutable_integer = _mutable_integer;

        // 报错! 当前作用域中 `_mutable_integer` 变量是不可变的
        _mutable_integer = 50;
        // FIXME ^ 注释这行代码

        // `_mutable_integer` 变量作用域结束
    }

    // `_mutable_integer` 这里可以修改，因为父作用域的定义是可变的
    _mutable_integer = 3;
}
