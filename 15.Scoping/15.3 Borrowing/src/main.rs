//!
//! 变量借用（Borrowing）
//!
//! 大多数情况下我们只想使用变量的数据并不是想转移变量的所有权，
//! `Rust` 提供了 `&` 关键字来创建一个引用，引用只是对数据的借用，而不会转移所有权，
//! 通过在变量前加上 `&` 关键字来创建一个引用。
//!

fn example01() {
    // 这个函数会转移传递进来的参数的所有权，函数执行完成以后就会销毁参数。
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // 这个函数只是对参数的借用，不会销毁参数。
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    // `Box` 是在堆中储存的，所以会应用所有权规则。
    let boxed_i32 = Box::new(5_i32);
    // `6_i32` 是在栈中储存的，栈中保存的数据会在所有权转移的时候自动复制一份。
    let stacked_i32 = 6_i32;

    // 这里只是对两个参数的借用，所以不会转移所有权，而且在之后的逻辑中可以继续使用这两个变量。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 手动创建一个引用。
        let _ref_to_i32: &i32 = &boxed_i32;

        // 错误！因为函数要求的是一个可以转移所有权的变量，而这里只提供了一个引用。
        // eat_box_i32(boxed_i32);
        // TODO ^ 移除注释查看错误

        // 这里符合函数要求所以可以使用
        borrow_i32(_ref_to_i32);

        // 当前的作用域结束了，所以 `_ref_to_i32` 会被销毁。
    }

    // 这里可以把 `boxed_i32` 传递进去，因为 `boxed_i32` 可以被转移，当函数执行完成以后，`boxed_i32` 就不能再次使用了。
    eat_box_i32(boxed_i32);
}

/// 可变变量的所有权转移和借用
fn mutability() {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        // `&'static str` 是一个只读的字符串引用，并且在整个程序的证明周期内都有效。
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    // 函数接收一个不可变引用
    fn borrow_book(book: &Book) {
        println!(
            "I immutably borrowed {} - {} edition",
            book.title, book.year
        );
    }

    // 函数接受一个可变的引用。
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    // 创建一个不可变的结构体实例
    let immutabook = Book {
        // 字符串的字面量是 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 通过所有权转移和重定义让 `immutabook` 变成一个可变的 `mutabook`
    // 因为 `Book` 实现了 `Copy` 和 `Clone` 特性，所以这里实际上是复制了一份数据，并没有所有权的转移。
    let mut mutabook = immutabook;

    // 从不可变对象借用一个不可变引用。
    borrow_book(&immutabook);

    // 从可变对象上借用一个不可变引用。
    borrow_book(&mutabook);

    // 从可变对象上借用一个可变引用。
    new_edition(&mut mutabook);

    // 错误！不能从不可变引用上创建可变引用。
    // new_edition(&mut immutabook);
    // TODO ^ 移除注释查看错误
}

/// 变量的借用规则
/// 一个数据可以同时被借用无数个不可变引用，但是一个数据同时只允许借用一个可变引用，
/// 当数据存在可变引用，则其他的任何引用都不允许使用。
/// 或者是数据存在不可变引用，这个不可变引用的使用周期内都不允许再次借用可变引用，
/// 只有不可变引用不再使用了以后才允许借用一个可变引用。
///
fn aliasing() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 使用不可变引用读取数据，这三个都指向同一份数据。
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 错误！`point` 在这里不允许借用一个可变的引用，因为下面的 `println!` 中使用了上面的不可变引用。
    // let mutable_borrow = &mut point;
    // TODO ^ 移除注释查看错误

    // The borrowed values are used again here
    // 因为这里使用了不可变引用导致了上面的可变引用的借用失败了。
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 上面的所有不可变引用在这里都已经不再使用了，所以这里可以借用一个可变的引用。
    let mutable_borrow = &mut point;

    // 通过可变引用修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 错误！因为当前 `point` 已经被借用了一个可变引用，所以不允许再次借用了。
    // let y = &point.y;
    // TODO ^ 移除注释查看错误

    // 错误！因为 `point` 已经存在了一个可变引用，所以这里不允许再借用一个不可变引用。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ 移除注释查看错误

    // Ok! Mutable references can be passed as immutable to `println!`
    // 这里可以使用对可变引用再次借用不可变引用。
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // 上面的可变引用已经不再使用了，所以这里可以重新借用一个不可变引用。
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

/// 很多场景下不能通过 `&` 关键字来定义引用，所以需要使用 `ref` 关键字
fn the_ref_pattern() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let c = 'Q';

    // `ref` 关键字可以使用在等号左侧代表借用右边的值。
    // `&` 关键字使用在等号右侧也是代表借用右边的值。
    // 两个关键字的效果是一样的，只不过使用的位置不同。
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` 关键字在解构解构的时候非常有用。
    let _copy_of_x = {
        // 强制让 `ref_to_x` 是一个不可变引用。
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // 返回具体的数据，因为 `x` 字段是一个基础值所以会自动复制数据
        *ref_to_x
    };

    // 对 `point` 进行所有权转移，这里实际上是 `Copy` 因为 `Point` 实现了 `Copy` 特性，
    // 然后重新绑定为可变对象。
    let mut mutable_point = point;

    {
        // `ref` 可以加 `mut` 修饰来表明是一个可变引用
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // 因为是可变引用，所以这里可以直接修改值
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // 定义一个可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 这里对元组进行解构，拿到一个可变的引用。
        let (_, ref mut last) = mutable_tuple;
        // 修改可变引用中的数据
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}

fn main() {
    // 基础示例
    example01();
    // 可变引用和不可变引用
    mutability();
    // 可变引用和不可变引用的借用关系
    aliasing();
    // `ref` 的使用场景
    the_ref_pattern();
}
