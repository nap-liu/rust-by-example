//!
//! `super` 和 `self` 关键字
//!
//! 这两个关键字可以让硬编码的模块路径写成相对的路径关系。
//!
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 在当前作用域中尝试访问所有的 `function` 函数！
        print!("called `my::indirect_call()`, that\n> ");

        // `self` 关键字明确的指出，我们要访问的作用域是当前的模块。
        // 所以 `self::function()` 指明了我们调用的函数是当前模块中定义的函数。
        self::function();

        // 如果没有指明作用域，那么作用域等同于 `self`。
        function();

        // 可以使用 `self` 关键字来明确的指出，我们访问的是当前模块下定义的 `my` 子模块。
        // `self` 关键字是可以省略的，因为当不指明作用域的时候等同于 `self`。
        self::cool::function();

        // `super` 关键字明确的指明了我们需要访问的是父模块中的方法，也就是当前的 `my` 模块的上一级。
        super::function();

        // 还可以通过 `crate` 特殊的作用域来重新从当前的包的根作用域下重新指定一个完整的路径。
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
