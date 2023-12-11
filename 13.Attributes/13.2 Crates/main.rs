//!
//! 属性定义还可以额外指定包的属性，比如说包名称，包的类型。
//!

// 通过属性指明当前的包是一个库
#![crate_type = "lib"]
// 通过属性指明当前的包的名称
#![crate_name = "rary"]

// 上面两个属性声明了当前的包的名称和类型，
// 这样使用 `rustc` 编译的时候就不需要手动指明类型和名称了。

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
