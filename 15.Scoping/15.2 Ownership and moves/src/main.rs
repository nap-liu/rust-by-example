//!
//! 拥有权和拥有权转移
//!
//! 因为变量负责释放自己的资源，所以资源的所有权只能由一个变量拥有，
//! 这个限制可以避免出现资源的多重释放，（对于资源的引用并不会拥有所有权）
//!
//! 当进行变量赋值 `(let x = y)` 或者传递参数 `foo(x)` 的时候，对于
//! 资源的所有权会被转移，使用 `Rust` 语言表达就是变量发生了转移。
//!
//! 当资源被转移以后，上一个拥有所有权的变量就不能再次被使用了，这个限制
//! 也避免了 `悬空指针(dangling pointers)` 的问题
//!
//!

fn ownership_and_moves() {
    // 这个函数会转移所有权，并且拥有堆上分配的内存
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // 作用域销毁 `c` 被回收，堆上的内存也被回收。
    }

    // 保存在栈上的数据
    let x = 5u32;

    // 复制了栈上的数据，这里没有用到堆数据，所以在这行代码之后 `x` 还可以继续使用
    let y = x;

    // 因为数据是保存在栈上的，所以这两个变量都是独立并且可用的。
    println!("x is {}, and y is {}", x, y);

    // 通过 `Box` 在堆上分配一块内存并保存数字 `5`，并把 `Box` 绑定给 `a` 变量
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // 转移 `a` 资源的所有权转移给 `b`
    let b = a;
    // 把 `a` 中保存的资源地址（这里只复制了指针，并没有复制堆上的数据）复制给 `b`，
    // 这时候 `a` 和 `b` 同时都指向堆上面的同一个的资源，
    // 但是因为所有权的限制当地址复制给 `b` 的时候我们称 `a` 指向的资源的所有权转移给了 `b`，
    // 这时候 `a` 就不再有效，只有 `b` 可以访问这个资源。

    // 错误！`a` 不能再使用了，因为所有权已经被转移了。
    // println!("a contains: {}", a);
    // TODO ^ 移除注释看错误

    // 这个函数会转移 `b` 的所有权，所以后面也不再能访问 `b` 了
    destroy_box(b);

    // 因为 `b` 的所有权已经转移给了函数 `destroy_box`，所以这里不能再次使用 `b` 了，
    // 错误！`b` 不能再使用了，因为所有权已经被转移了。
    //println!("b contains: {}", b);
    // TODO ^ 移除注释看错误
}

/// 可以在变量的所有权转移的时候让数据可以修改
fn mutability() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 错误! 不可变数据不能修改
    // *immutable_box = 4;

    // 转移变量所有权的时候可以重新修改变量为可变的
    // 也就是通过 `let mut x = x` 这样等于通过所有权转移 + 变量遮蔽 让变量变成了可变的。
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 通过解引用操作符获取可变引用修改数值
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

/// 使用解构操作符的时候可以在一个解构语句中同时使用 `转移（所有权转移）` 和 `引用（使用 ref 关键字）`，
/// 这样的话解构操作完成以后，被转移出来的属性在原始对象上就不能再次使用了，解构出来的引用是可以继续再原始对象上用的。
fn partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` 字段被转移出来了，`age` 因为使用了 `ref` 关键字只是借用了一个不变的引用。
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 错误！因为 `name` 字段已经被转移出来了，所以这里不能再把 `person` 当做完整的对象使用。
    // println!("The person struct is {:?}", person);

    // `person` 不能完整的使用，但是可以使用 `person.age` 因为这个属性并没有被转移。
    println!("The person's age from person struct is {}", person.age);
}

fn main() {
    ownership_and_moves();
    mutability();
    partial_moves();
}
