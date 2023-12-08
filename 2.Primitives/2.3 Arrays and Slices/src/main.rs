use std::mem;

// 该函数借用数组切片（slice）
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // 固定长度和类型的数组，这里可以不声明类型和长度，编译器会自动推断
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 数组类型是 i32 长度是 500，所有 500 个元素初始化为 0.
    let ys: [i32; 500] = [0; 500];

    // 数组的索引从 0 开始
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len()` 方法会返回数组的长度。
    println!("Number of elements in array: {}", xs.len());

    // 因为数组的类型和长度是固定的，编译的时候会直接把数组放在栈上。
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动创建一个对于该数组的切片（slice）
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // 切片（slice）可以通过指定一个区间 [起始索引..结束索引] 来借用数组的一部分数据
    // `起始索引` 是切片的第一个元素的位置
    // `结束索引` 是切片的最后一个元素的位置 + 1 的位置
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // 创建一个空的切片 `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]); // 对空数组创建一个切片引用，实际上这个等于下面的代码。
    assert_eq!(&empty_array, &[][..]); // 对空数组创建切片引用，引用范围为整个数组。

    // 数组还可以通过 `.get()` 方法安全的访问，该方法会返回一个 `Option`
    // 可以通过 `.expect()` 方法来输出一个明确的错误。
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // 因为数组类型是明确长度和类型的，所以在编译阶段会进行数组的访问越界检查，
    // 一旦发现越界就会报出错误，无法通过编译。
    //println!("{}", xs[5]);

    // 切片类型在编译阶段是不能知道长度的，
    // 所以切片会在运行的过程中检查访问的索引是否越界，一旦越界就会产生 `panic!` 错误。
    //println!("{}", xs[..][5]);
}
