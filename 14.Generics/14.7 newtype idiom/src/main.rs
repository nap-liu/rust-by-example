//!
//! 新类型 （newtype）
//!
//! 新类型是指使用元组结构体包装一个其他的类型，然后基于新包装的类型实现类型的扩展或约束，
//! 这样的好处是可以让 `api` 更加清晰，不会造成歧义。
//!

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// 舍弃不足的年份
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));

    {
        let years = Years(42);
        let years_as_primitive_1: i64 = years.0; // 访问元组数据
        let Years(years_as_primitive_2) = years; // 解构元组数据
    }
}
