// 把 List 枚举定义的所有项都导入到全局
use crate::List::*;
enum List {
    // Cons 定义为元组类型，第一个位是值，第二位是下一个值的指针
    Cons(u32, Box<List>),
    // Nil 表示链表结束 没有下一项了
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        // 生成一个新的 Cons 把当前的链表放到新的 Cons 中
        Cons(elem, Box::new(self))
    }

    // 计算链表长度
    fn len(&self) -> usize {
        // 因为 Rust 会自动进行解引用操作 所以这里可以直接使用 self 而不是使用 *self
        match self {
            // 递归计算所有链表节点
            Cons(_, next) => 1 + next.len(),
            // 末尾节点没有值直接返回 0
            Nil => 0,
        }
    }

    // 格式化链表成字符串
    fn stringify(&self) -> String {
        // *号解引用会转移变量所有权，但是因为参数上声明是个引用
        // 所以这里会报错，不能把值从引用中转移出来
        // 解决方案就是在声明语句前加上 ref 关键字来表示变量是引用形式使用的

        match *self {
            // 这里使用 ref 关键字来声明变量是一个引用，而不是转移变量的所有权
            // head 不需要添加 ref 关键字使用为 head 是一个 i32 的基础值，直接保存在栈上
            // 而 Box<List> 的值是保存在堆上的。
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 从前面开始添加一些节点
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 查看链表的状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
