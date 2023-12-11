//!
//! 使用 `use` 声明
//!
//! `use` 关键字可以把一个完整的路径在当前的作用域中，
//! 重新绑定到一个新的名字，这样可以减少完整路径的使用。
//!
//! 常用的几种形式如下
//!
//! ```
//! {
//!     // 把 `std::rc::Rc` 使用 `Rc` 这个名字绑定到当前的作用域下
//!     use std::rc::Rc;
//! }
//!
//! {
//!     // 把 `std::rc::Rc` 使用 `StdRc` 这个名字绑定到当前的作用域下
//!     use std::rc::Rc as StdRc;    
//! }
//!
//! {
//!     // 把 `std::rc` 下的所有导出项全都绑定到当前作用域下
//!     use std::rc::*;
//! }
//!
//! {
//!     // `self` 关键字代表的是前一级的路径，
//!     // 在这里 `self` 等于把 `std::rc` 使用 `rc` 这个关键字绑定到当前作用域，
//!     // 同时把 `std::rc::Rc` 使用 `StdRc` 绑定到当前作用域，
//!     // 和 `std::rc::Weak` 使用 `Weak` 绑定到当前作用域。
//!     use std::rc::{self, Rc as StdRc, Weak};
//! }
//!
//! ```
//!

// 把 `deeply::nested::function` 绑定到 `other_function`。
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // 使用重新绑定的名字调用 `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // `crate` 代表当前的包，这里相当于把 `crate::deeply::nested::function` 重新绑定到当前作用域，名字叫 `function`，
        // 因为 `shadowing(遮蔽)` 的规则，`use` 关键字重新绑定的 `function` 会覆盖掉当前的全局作用域下的 `function` 函数。
        use crate::deeply::nested::function;

        // 这里调用的函数实际上是 `deeply::nested::function`，
        // 因为 `use` 关键字的重新绑定在当前的嵌套作用域中遮蔽了全局作用域下的 `function`
        function();

        println!("Leaving block");
    }

    function();
}
