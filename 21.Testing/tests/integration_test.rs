//!
//! 集成测试（Integration testing）
//!
//! 单元测试可以让我们对单个模块和内部的代码分别的进行测试，
//! 集成测试是用来对整个包（crate）的公开功能进行测试，
//! 来保证对外公开的功能是符合预期的。
//!
//! 集成测试的所有代码都被放置在根目录下的 `tests` 目录中，
//! 该目录下的所有代码都会被当做一个集成测试的单元，很多情况下
//! 需要定义一些公共的方法来进行一些测试的初始化，因为这个目录下的所有代码
//! 都会被当做集成测试的具体实现，为了同时支持独立文件集成测试和公共的测试代码抽取，
//! 所以这里可以使用老式的模块定义形式，也就是 `tests/somemodule/mod.rs` 来定义
//! 一个公共的测试模块，这样的话测试的过程中这种老模块声明的模块不会被当做集成测试的入口，
//! 但是这个模块定义的方法可以被其他的继承测试使用。
//!
//!

///
/// 使用老模块的文件结构声明的模块 不会被当做集成测试的代码
/// 基于这样的特性
///
mod common; // 引入公共的测试代码，包含一些公共的方法。

use common::*;
use testing::*;

#[test]
fn test_add() {
    setup();
    assert_eq!(add(3, 2), 5);
}
