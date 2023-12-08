fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 储存在栈上的基础值，会直接复制
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 没有被使用的变量编译器会发出警告，可以通过在变量前面添加一个下划线来禁用编译器警告
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ 在变量名前面添加下划线来禁用警告
    // PS：网页的在线测试看不到警告信息
}
