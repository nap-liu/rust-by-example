// 把 `inaccessible.rs` 或 `inaccessible/mod.rs` 当做一个子模块引用进来，但是该模块对外不可见。
mod inaccessible;
// 把 `nested.rs` 或 `nested/mod.rs` 当做一个子模块引用进来，并且对外公开该模块。
pub mod nested;

// 定义公开方法
pub fn function() {
    println!("called `my::function()`");
}

// 私有方法
fn private_function() {
    println!("called `my::private_function()`");
}

// 公开方法中引用私有方法。
pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
