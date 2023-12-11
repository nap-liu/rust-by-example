//!
//! # 模块化
//!
//! Rust 提供了非常强大的模块化系统，可以按照等级拆分代码到小的模块中。
//! 模块中还可以通过 `pub` 关键字控制代码是否对外可见。
//!
//! 模块中可以放置 函数(function)、结构体(struct)、特性(trait)、和实现(impl)、甚至其他的模块(module)
//!

/// 默认情况下模块内部的所有内容都是私有(private)的，
/// 需要通过 `pub` 关键字来让私有变成外部可访问的。
// 模块名为 my_mod 的模块
mod my_mod {
    // 模块内部的私有函数，模块外部不可见，也不能使用。
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 关键字让 `function` 函数外面可以调用。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 公开的函数内部可以使用当前模块内的任意方法，不管使用的函数是不是公开的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块可以嵌套使用。
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 可以通过 `pub(in path)` 的语法来指定某些方法，只允许指定的父模块访问，
        // `path` 只能是父模块或者祖先模块。
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // 如果使用 `pub(self)` 关键字定义的话，那就表示该函数只能在当前模块内部使用,
        // 等同于 private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // `pub(super)` 等同于当前的父模块，`super` 关键字就是父模块的别名
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) 表示这个函数只允许在当前的包内可用，`crate` 等于当前的包
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // 嵌套的模块依旧遵循私有规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // 因为当前模块没有对外公开，所以即使私有模块内部的方法公开了访问作用域，依旧还是不可见的。
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 因为模块有独立的作用域，所以可以避免同名函数的冲突问题。
    function(); // 全局作用域的函数

    my_mod::function(); // 模块内部的函数

    // 在模块外面可以通过模块的名称，使用 `::` 作用域访问关键字访问模块内部的方法，或者嵌套在模块内部的模块。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // 使用 `pub(crate)` 声明的方法，只要在同一个包（crate）中就可以随意使用。
    my_mod::public_function_in_crate();

    // 在模块中通过 `pub(in path)` 声明的方法，只能在 `path` 限定的地方使用
    // 报错！函数 `public_function_in_my_mod` 是私有的
    // my_mod::nested::public_function_in_my_mod();
    // TODO ^ 移除注释查看错误

    // 模块的私有项在外部不管什么情况下都不能被直接访问

    // 报错! `private_function` 是私有的
    //my_mod::private_function();
    // TODO ^ 移除注释查看错误

    // 报错! `private_function` 是私有的
    //my_mod::nested::private_function();
    // TODO ^ 移除注释查看错误

    // 报错! `private_nested` 是私有模块
    //my_mod::private_nested::function();
    // TODO ^ 移除注释查看错误

    // 报错! `private_nested` 是私有模块
    //my_mod::private_nested::restricted_function();
    // TODO ^ 移除注释查看错误
}
