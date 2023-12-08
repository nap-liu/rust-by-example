// 禁用未使用的代码警告
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // crate 关键字等于当前的根作用域。
    // 使用 `use` 关键字把指定类型引用到当前作用域，
    // 这样每次使用导入的类型的时候就不需要明确指定完整的路径了
    use crate::Status::{Poor, Rich};
    // 可以使用通配符 `*` 把对应作用域下的所有导出内容都导入到当前作用域下。
    use crate::Work::*;

    // 等于 Status::Poor
    let status = Poor;
    // 等于 Work::Civilian
    let work = Civilian;

    match status {
        // 因为前面明确的使用 `use` 关键字把 `Status` 导入到了当前作用域，所以这里不需要写 `Status::`
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 同上
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
