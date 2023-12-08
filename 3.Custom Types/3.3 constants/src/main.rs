// 全局定义常量
static LANGUAGE: &str = "Rust"; // 可以被修改的常量，修改常量是不安全的
const THRESHOLD: i32 = 10; // 不能修改的常量

fn is_big(n: i32) -> bool {
    // 全局的常量可以在任意位置访问
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在主线程访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错! 不能修改常量的值
    THRESHOLD = 5;
    // FIXME ^ 注释掉这行代码
}
