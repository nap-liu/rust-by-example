// 元组可以当做函数参数，也可以当做函数返回值使用
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` 关键字可以绑定元组的数据到变量上。
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// 该结构用于下面代码示例
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    // 元组可以容纳任意数量的任意类型值
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // 元组可以通过索引直接访问对应位置上的数据。
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // 元组可以嵌套使用
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组默认是可打印的
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但是元组的长度超过12个元素的话就不能打印了。
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ 移除上面两行代码的注释看看编译错误

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // 因为元组和表达式优先级使用了相同的语法。
    // 所以当想创建只有一个元素的元组类型，需要在元素后面加一个逗号来区分是元组还是表达式
    println!("One element tuple: {:?}", (5u32,));
    // 这个是优先级的表达式，返回一个整数
    println!("Just an integer: {:?}", (5u32));

    // 元组可以使用 `let` 来进行结构批量赋值给变量。
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
