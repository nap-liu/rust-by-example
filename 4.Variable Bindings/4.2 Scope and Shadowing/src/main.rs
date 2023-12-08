fn example01() {
    // 在当前函数作用域中 绑定 1 到变量 long_lived_binding
    let long_lived_binding = 1;

    // 这个是一个内部嵌套的小范围作用域
    {
        // 这个变量声明只在当前的小范围内生效
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // 嵌套的作用域结束，该作用域内的全部变量都会被回收

    // 报错! `short_lived_binding` 这个变量已经被回收了
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ 注释这行代码

    println!("outer long: {}", long_lived_binding);
}

fn example02() {
    let shadowed_binding = 1;

    {
        // 这里使用的变量值是外面的函数作用域的值
        println!("before being shadowed: {}", shadowed_binding);

        // 这里重新定义了一个同名的变量，进行了变量的遮蔽(shadowing)
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    // 小作用域已经结束，这里的变量使用的依旧是函数作用域的值
    println!("outside inner block: {}", shadowed_binding);

    // 这里对上面定义的变量重新定义，遮蔽（shadowing）了前面的变量声明
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn main() {
    example01();
    example02();
    println!("Hello, world!");
}
