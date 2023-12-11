// 公开发方法
pub fn function() {
    println!("called `my::nested::function()`");
}

// 私有方法
#[allow(dead_code)] // 这个是为了禁用未使用的警告，因为该函数是私有的，但是又没有任何地方访问它。
fn private_function() {
    println!("called `my::nested::private_function()`");
}
