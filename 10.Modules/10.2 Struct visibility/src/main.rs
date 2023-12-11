//!
//! 模块中结构体的可见性
//!
//! 结构体的字段拥有一个额外的可见性配置，
//! 默认情况下结构体内部的字段对外都是不可见的，
//! 需要在每个字段上手动指定 `pub` 来让外部可以访问指定的字段，
//!
//!

mod my {
    // 这个结构体对外可见
    pub struct OpenBox<T> {
        // 这个字段也对外可见
        pub contents: T,
    }

    // 这个结构体对外可见
    pub struct ClosedBox<T> {
        // 这个字段外面不可见
        #[allow(dead_code)]
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 结构体上拥有一个 `new` 方法是对外可见的
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn main() {
    // 如果结构体可见并且所有的字段也都对外可见的话，则可以直接声明字面量就能的到结构体实例
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // 结构体的字段可以直接访问
    println!("The open box contains: {}", open_box.contents);

    // 公开的结构体如果有私有的字段，则不能直接使用字面量形式构造实例，因为私有字段对外不可见。
    // 错误！`ClosedBox` 拥有不可见的私有字段。
    // let closed_box = my::ClosedBox {
    //     contents: "classified information",
    // };
    // TODO ^ 移除注释查看错误

    // 拥有私有结构的结构体可以通过结构上的公开的关联函数来构造结构体的实例。
    let _closed_box = my::ClosedBox::new("classified information");

    // 结构体的私有字段外部是不能访问的。
    // 错误！ `contents` 字段是私有的
    // println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ 移除注释查看错误
}
