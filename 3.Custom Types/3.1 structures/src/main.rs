// 禁用未使用的代码警告
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单位结构 类似 单位元组(Unit Tuple)
// 虽然和单位元组类似，但是每一个 struct 都是独一无二的。
struct Unit;

// 元组结构
struct Pair(i32, f32);

// 两个字段的结构
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以嵌套使用
#[derive(Debug)]
struct Rectangle {
    // 长方形可以使用 top、left 和 bottom、right 两个点来表示
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f64 {
        // 解构两个坐标点
        let Rectangle {
            top_left,
            bottom_right,
        } = self;
        // 计算宽高
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;
        // 计算面积
        width as f64 * height as f64
    }

    fn square(top_left: &Point, size: f32) -> Rectangle {
        Rectangle {
            // 复制 Point 结构体
            top_left: Point { ..*top_left },
            // 按照尺寸对左上角的坐标进行扩大
            bottom_right: Point {
                x: top_left.x + size,
                y: top_left.y + size,
            },
        }
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    // 如果变量和结构体的字段名正好相同，则可以省略字段名
    let peter = Person { name, age };

    // 使用 fmt::Debug 来打印结构体
    println!("{:?}", peter);

    // 实例化 `Point` 结构体
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 通过字段访问结构体
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用一个已经存在的结构体的内容来补充剩余的字段，创建一个新的结构体。
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`

    // `bottom_right.y` 和 `point.y` 的值相同，
    // 因为 `button_right.y` 的值是从 `point` 上复制过来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // `let` 也可以对结构体进行结构赋值
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 实例化结构体也是一个表达式，该表达式返回新的结构体实例
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // 实例化一个 `Unit` 结构
    let _unit = Unit;

    // 实例化一个 元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体和元组本身的访问行为相同
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构元组结构体同样和解构元组相同
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let square = Rectangle::square(&point, 100f32);

    println!("square: \n{:#?}", square);
    println!("square area: {}", square.rect_area());
}
