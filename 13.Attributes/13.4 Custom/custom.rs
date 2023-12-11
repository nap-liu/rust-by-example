//! cfg 还支持自定义属性值
//! 通过 `rustc --cfg custom` 传递额外的值
//!

// 这个属性就是外部传递的
// rustc --cfg some_condition custom.rs && ./custom
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
