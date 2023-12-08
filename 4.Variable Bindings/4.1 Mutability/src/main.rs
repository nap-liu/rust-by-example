fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 可变声明，所以可以修改
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 报错! 不可变声明不能修改内容
    _immutable_binding += 1;
}
