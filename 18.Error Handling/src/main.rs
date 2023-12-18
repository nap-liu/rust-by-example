//!
//! 错误处理
//!
//! 错误处理是为了处理程序中可能出现的错误，比如说读取文件的过程中出错，如果继续使用这个错误的结果
//! 进行后续的处理肯定是错误的，应该明确的处理这些错误，并且避免其他部分使用这种错误的数据。
//!
//! Rust 中有很多种方法来处理错误，下面的章节会逐步讲解这些处理方法，它们之间会有一些细微的差别，
//! 和不同的应用场景，下面是一些错误处理的经验。
//!
//! 在测试用例的代码中可以使用 `panic!` 来处理不可恢复性的错误让测试使用具体的信息报出错误。
//! 在设计阶段可以也可以使用 `panic` 的提示来告诉用户某些方法还没有实现。
//! 在测试阶段使用 `panic!` 是非常有效的一种错误提示。
//!
//! 还有一种类型是 `Option` 他代表了一种可选地数据类型，当某个值是可选地时候缺少具体的值不应该是一个错误，
//! 常见的场景是目录当要获取 `/` 或 `C:` 的父目录的时候是获取不到的，因为当前目录已经是根目录了，而这个操作不应该 `panic!`。
//! 当使用 `Option` 类型的时候如果能确定值一定存在则可以使用 `unwrap` 方法来获取值，
//! 如果说当值不存在的时候必须要 `panic!` 的时候可以使用 `expect` 方法来快捷的指定 `panic!` 的错误信息。
//!
//! 如果使用方必须处理可能出现的错误的情况下可以使用 `Result` 类型，使用方就可以是 `expect` 和 `unwrap` 方法来处理结果
//! （通常情况下不要使用这俩方法，因为一旦有错误产生就会 `panic` 可以使用解构语法来避免程序崩溃）。
//!
//! 其他的错误处理情况可以参考[官方推荐的书中的错误处理方式](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
//!

/// 最简单的处理错误方法就是使用 `panic!`，它可以明确的给出错误信息和崩溃前的调用栈，
/// 可以明确的使用 `panic!` 宏来让程序马上崩溃退出。
fn panic_() {
    fn drink(beverage: &str) {
        // 你不应该喝太多的含糖饮料
        if beverage == "lemonade" {
            // 明确调用 `panic!` 宏来让程序崩溃退出。
            panic!("AAAaaaaa!!!!");
        }

        println!("Some refreshing {} is all I need.", beverage);
    }

    drink("water");
    drink("lemonade");
    // 下面这个方法永远都不会被调用，因为上面的方法造成了 `panic!`
    drink("still water");
}

fn abort_and_unwind() {
    fn drink_with_cfg_macro(beverage: &str) {
        // 你不应该喝太多的含糖饮料
        if beverage == "lemonade" {
            // 使用条件宏来定义 `panic` 的行为
            // 条件宏会把所有分支的代码都编译到结果中
            if cfg!(panic = "abort") {
                println!("This is not your party. Run!!!!");
            } else {
                println!("Spit it out!!!!");
            }
        } else {
            println!("Some refreshing {} is all I need.", beverage);
        }
    }

    // 属性宏会在编译阶段就确定使用哪种代码，没有命中的代码不会被编译到可执行文件中
    // 使用属性宏来调整代码行为
    #[cfg(panic = "unwind")]
    fn ah() {
        println!("Spit it out!!!!");
    }

    // 使用属性宏来调整代码行为
    #[cfg(not(panic = "unwind"))]
    fn ah() {
        println!("This is not your party. Run!!!!");
    }

    fn drink_with_attr_macro(beverage: &str) {
        if beverage == "lemonade" {
            ah();
        } else {
            println!("Some refreshing {} is all I need.", beverage);
        }
    }

    drink_with_attr_macro("water");
    drink_with_attr_macro("lemonade");
}

/// `Option` 类型和内置方法
///
/// `Option<T>` 是一个枚举类型，拥有两个枚举值 `Some<T>` 和 `None`，
/// 这两个选项可以使用 `match` 明确的处理每种可能性，
/// 或者使用 `unwrap` 来获取值，当没有值的时候会自动调用 `panic!`。
/// 还可以使用 `expect` 方法来明确的指定没有值的时候 `panic!` 抛出的错误信息。
///
fn option_and_unwrap() {
    // 让成年人来判断喝的东西是什么。
    fn give_adult(drink: Option<&str>) {
        // 这里使用了 `match` 关键字来处理 `Option` 的每一种可能性。
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No drink? Oh well."),
        }
    }

    // 如果尝试去拿要喝的东西，如果拿不到的话 `unwrap` 方法会直接 `panic!`，
    // 如果拿到的喝的东西是 `lemonade` 的话也会 `panic!`。
    // 其余的都可以喝。
    fn drink(drink: Option<&str>) {
        // `unwrap` 方法会检查 `drink` 是不是 `None` 如果是的话会 `panic!`。
        let inside = drink.unwrap();

        // 这里检查是不是 `lemonade` 如果是的话也会 `panic!`。
        if inside == "lemonade" {
            panic!("AAAaaaaa!!!!");
        }

        println!("I love {}s!!!!!", inside);
    }

    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    // 这里会 `panic!`
    drink(nothing);
}

/// 使用 `?` 号操作符来获取 `Option` 中存放的数据。
///
/// `Option` 可以使用 `match`，`if let` 语句来获取数据，还可以使用 `?` 操作符来更快捷的获取数据，
/// 当 `x = Some(T)` 的时候可以使用 `let value = x?` 来获取 `Some(T)` 中的 `T`，也就是说
/// 这里的 `value = T`，如果 `x = None` 的话则当前的函数会提前退出并把 `None` 返回给调用方。///
///
fn unpacking_options_with_question_mark() {
    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // 如果 `current_age` 是 `None` 直接返回 `None`.
        // 如果 `current_age` 是 `Some` 内部的 `u8` 经过计算后会赋值给 `next_age`
        let next_age: u8 = current_age? + 1;
        Some(format!("Next year I will be {}", next_age))
    }

    // `?` 号操作符支持链式使用
    struct Person {
        job: Option<Job>,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }

    impl Person {
        // 获取当前结构上的指定值。
        fn work_phone_area_code(&self) -> Option<u8> {
            // 这里使用了链式的 `?` 操作符来快捷的从 `Option` 中获取数据，如果数据不存在的话则会返回 `None`
            // 相比使用 `match` 语句来说 `?` 更加方便快捷
            self.job?.phone_number?.area_code
        }
    }

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}

/// `match` 可以处理 `Option`，但是当层级特别深或者链路很长的时候写起来会非常麻烦，
/// 尤其当只需要处理一个可能的有效值的时候极其麻烦。
///
/// 所以 `Option` 提供了一个方法是 `map`，用于只处理有值的情况。
///
fn combinators_map() {
    #![allow(dead_code)]

    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    // 使用 `match` 方法处理 `Option`
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // 使用 `match` 方法处理 `Option`
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    // 这里使用 `map` 操作符来替换 `match` 关键字
    // 可以看到相同的逻辑使用 `map` 更加简单
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // 使用链式语法一次性做多次转换
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    // 使用 `match` 操作符对结果做不通的处理
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm. I love {:?}", food),
            None => println!("Oh no! It wasn't edible."),
        }
    }

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

///
/// `map` 可以使用链式语法来简化 `match` 语句，
/// 但是使用 `map` 如果闭包返回的是 `Option<T>`
/// 则会导致结果会被包装成 `Option<Option<<T>>`
/// 因为 map 会对闭包的返回值进行包装，当链式调用很多的话可能会让代码变得不可读。
///
/// 所以 `Option` 还提供了另一个方法叫 `and_then` 在其他语言下叫 `flatmap`，
///
/// 如果调用 `and_then` 的对象是一个 Option 的包装对象，则会返回包装的内容，
/// 如果是 `None` 则会直接返回 `None`
///
///
fn combinators_and_then() {
    #![allow(dead_code)]

    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }
    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    fn have_ingredients(food: Food) -> Option<Food> {
        match food {
            Food::Sushi => None,
            _ => Some(food),
        }
    }

    fn have_recipe(food: Food) -> Option<Food> {
        match food {
            Food::CordonBleu => None,
            _ => Some(food),
        }
    }

    fn cookable_v1(food: Food) -> Option<Food> {
        match have_recipe(food) {
            None => None,
            Some(food) => have_ingredients(food),
        }
    }

    // `and_then` 的返回值就是闭包的返回值，不会进行包装。
    fn cookable_v3(food: Food) -> Option<Food> {
        have_recipe(food).and_then(have_ingredients)
    }

    // `map` 会对返回值进行包装，然后使用 `flatten` 方法展开一层 `Option` 的包装
    fn cookable_v2(food: Food) -> Option<Food> {
        have_recipe(food).map(have_ingredients).flatten()
    }

    fn eat(food: Food, day: Day) {
        match cookable_v3(food) {
            Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
            None => println!("Oh no. We don't get to eat on {:?}?", day),
        }
    }

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}

///
/// `Option` 有很多方法可以获取内部封装的值，如果没有值的情况下可以使用 `None` 来当做默认值。
/// 你可以根据不同的场景来选择使用不同的方法。
///
/// - 是立即还是可以延迟计算？
/// - 是否需要对数据已经转移并且让原始位置清空，或者在原始的位置直接修改？
///
fn unpacking_options_and_default() {
    // `or` 方法是一个可以链式、立即计算、并且把原始值直接转移出来的方法
    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kiwi,
            Lemon,
        }

        let apple = Some(Fruit::Apple);
        let orange = Some(Fruit::Orange);
        let no_fruit: Option<Fruit> = None;

        let first_available_fruit = no_fruit.or(orange).or(apple);

        println!("first_available_fruit: {:?}", first_available_fruit);
        // first_available_fruit: Some(Orange)

        // `or` 会同时转移调用示例和传递的参数的所有权。
        // 上面的调用中同时使用了 `apple`，`orange`，`no_fruit` 三个变量，这三个变量的所有权都被转移了。
        // 所以这里不能再使用这些变量了。
        // println!(
        //     "Variable apple was moved, so this line won't compile: {:?}",
        //     apple
        // );
        // TODO: 移除注释查看错误信息
    }

    // `or_else` 方法是一个可以链式、惰性计算、并且把原始值直接转移出来的方法
    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kiwi,
            Lemon,
        }

        let apple = Some(Fruit::Apple);
        let no_fruit: Option<Fruit> = None;

        // 定义默认值的闭包函数
        let get_kiwi_as_fallback = || {
            println!("Providing kiwi as fallback");
            Some(Fruit::Kiwi)
        };
        // 定义默认值的闭包函数
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Some(Fruit::Lemon)
        };

        let first_available_fruit = no_fruit
            .or_else(get_kiwi_as_fallback) // 传递默认值闭包函数
            .or_else(get_lemon_as_fallback); // 传递默认值闭包函数
        println!("first_available_fruit: {:?}", first_available_fruit);
        // first_available_fruit: Some(Kiwi)
    }

    // `get_or_insert` 方法是一个立即计算，当值为 `None` 的时候在原地插入默认值的方法
    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kiwi,
            Lemon,
        }

        let mut my_fruit: Option<Fruit> = None;
        let apple = Fruit::Apple;
        let first_available_fruit = my_fruit.get_or_insert(apple);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
        // first_available_fruit is: Apple
        // my_fruit is: Some(Apple)

        // 这里会报错因为 `apple` 的所有权被转移了。
        // println!("Variable named `apple` is moved: {:?}", apple);
        // TODO: 解除注释查看错误
    }

    // `get_or_insert_with` 方法是一个惰性计算，并且值为 `None` 的时候原地修改值的方法。
    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kiwi,
            Lemon,
        }
        let mut my_fruit: Option<Fruit> = None;

        // 定义默认值函数
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Fruit::Lemon
        };

        // 给出默认值函数
        let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
        // Providing lemon as fallback
        // first_available_fruit is: Lemon
        // my_fruit is: Some(Lemon)

        let mut my_apple = Some(Fruit::Apple);
        // 如果调用 `get_or_insert_with` 方法的值不为 `None` 的话，则直接使用原始值。
        let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
        println!("should_be_apple is: {:?}", should_be_apple);
        println!("my_apple is unchanged: {:?}", my_apple);
        // should_be_apple is: Apple
        // my_apple is unchanged: Some(Apple)
    }
}

///
/// `Result` 对象
/// `Result` 是 `Option` 对象更加明确的对于结果的描述对象，该对象使用了 `Result<T, E>` 签名来描述，
/// 更加明确的描述了一个操作的结果是正确还是错误。
///
/// 下面是 `Result<T, E>` 对象的两个可能的值
///
/// - `Ok(T)` 拥有一个正确的结果
/// - `Err(E)` 拥有一个错误的结果
///
/// `Result` 和 `Option` 非常相似，拥有很多相同的方法比如说 `unwrap`，当结果是一个 `Err(E)` 的时候则会调用 `panic!`。
/// 大多数 `Option` 上的方法在 `Result` 都可以直接使用。
///
fn result() {
    fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
        // 尝试 `unwrap` `parse` 方法返回的结果。
        let first_number = first_number_str.parse::<i32>().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();
        first_number * second_number
    }

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // 这里会 `panic!` 因为 `t` 不能被转换成数字。
    // let tt = multiply("t", "2");
    // println!("double is {}", tt);
    // TODO: 移除注释查看错误
}

///
/// 在代码中 `main` 函数是一个非常特殊的函数，该函数可以和 `C/C++` 的行为一样拥有返回值，
/// 在 `Rust` 中 `main` 函数可以通过 `Result` 对象来定义错误的返回值，该错误需要能被 `{:?}`
/// 符号格式化，也就是说需要实现 `Debug` 特性。
///
///
/// 注意 下面这个函数中的代码只能在独立的工程中使用，因为一个程序只允许存在一个 `main` 函数。
///
fn using_result_in_main() {
    // 使用这些代码重新创建一个工程来查看 `main` 函数拥有返回值的效果
    use std::num::ParseIntError;
    fn main() -> Result<(), ParseIntError> {
        let number_str = "10";
        let number = match number_str.parse::<i32>() {
            Ok(number) => number,
            Err(e) => return Err(e),
        };
        println!("{}", number);
        Ok(())
    }
}

///
/// `Result` 对象的 `map` 方法
///
/// 在 `fn result();` 函数中的 `multiply` 方法因为使用了 `unwrap` 导致了可能会让程序崩溃，所以代码不够健壮，
/// 通常我们更希望函数会把错误告知给调用方，让调用方来决定应该如何处理对应的错误。
///
/// 第一点我们需要先知道错误的类型是什么，然后决定对应的错误类型进行针对性的处理，
/// 就像是 `multiply` 函数中可能出错的方法是 `parse` 所以我们可以通过查看 `parse` 的定义来查看错误的类型。
///
fn map_for_result() {
    use std::num::ParseIntError;

    // 通过查看 `parse` 方法找到了对应的错误类型是 `ParseIntError`，
    // 然后我们修改代码为 `match` 然后在修改函数签名和代码逻辑，当出现错误的时候直接返回错误。
    fn multiply_with_match(
        first_number_str: &str,
        second_number_str: &str,
    ) -> Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => match second_number_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // 大多数的 `Option` 方法都可以使用在 `Result` 上，
    // 所以这里可以使用这些快捷方法来实现相同的功能，但是需要的代码更少，逻辑更清晰
    fn multiply_with_combinators(
        first_number_str: &str,
        second_number_str: &str,
    ) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    // 这里拿到一个结果的包装对象，包含了可能的结果。
    let twenty = multiply_with_match("10", "2");
    print(twenty);

    // 同上
    let tt = multiply_with_combinators("t", "2");

    // 通过辅助函数来输出结果的内容
    print(tt);
}

///
/// 使用类型别名来重新定义 `Result` 类型
/// 因为 `Result` 的签名要求我们需要明确的给出两个泛型类型才能正常使用，这样的话每次使用的时候会有点麻烦
/// 我们可以通过类型别名来缩减代码，重新定义一个只有一个泛型参数的 `Result` 类型来简化我们的代码。
///
/// 在标准库中有一个非常常用的 `Result` 类型是 `std::io::Result` 实际上就是重新对 `std::result::Result`
/// 定义了一个别名 `type std::io::Result<T> = std::result::Result<T, std::io::Error>`
///
fn aliases_for_result() {
    use std::num::ParseIntError;

    // 定义了一个泛型的别名，把 `ParseIntError` 直接当做了 `Result` 的错误类型。
    type AliasedResult<T> = Result<T, ParseIntError>;

    // 这里修改返回值定义为我们自己定义的别名
    fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    // 这里修改输入类型为自定义的别名。
    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

/// 在前面的代码中我们使用了 `match` 关键字来处理各种情况，
/// 实际上在 `match` 中还可以通过使用 `return` 关键字来提前退出函数。
fn eraly_result() {
    use std::num::ParseIntError;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(first_number) => first_number,
            Err(e) => return Err(e), // 使用 `return` 关键字来提前退出函数的执行并返回一个值
        };

        let second_number = match second_number_str.parse::<i32>() {
            Ok(second_number) => second_number,
            Err(e) => return Err(e), // 使用 `return` 关键字来提前退出函数的执行并返回一个值
        };

        Ok(first_number * second_number)
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

///
/// 通常情况下我们只需要获取内部封装的数据，如果遇到错误的话就直接返回错误，而不是让程序崩溃
/// 而 `?` 操作符就是专门为了解决这个场景而设计的，
///
/// 来看看使用 `?` 操作符来简化之前写的这个代码。
///
fn intorducing_question_mark() {
    // `?` 操作符
    {
        use std::num::ParseIntError;

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            // 使用 `?` 操作符替换掉 `match` 或 `unwrap` 等等的一系列操作符
            let first_number = first_number_str.parse::<i32>()?;
            let second_number = second_number_str.parse::<i32>()?;

            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }

    // // `?` 操作符的前身 `try!` 宏
    // {
    //     // 这个地方会报错因为 `try!` 宏已经被 `?` 操作符取代了，
    //     // 想要能正常编译需要修改 `Cargo.toml` 中 `[package]` 下面的 `edition` 字段为 `2015`
    //     use std::num::ParseIntError;

    //     fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    //         let first_number = try!(first_number_str.parse::<i32>());
    //         let second_number = try!(second_number_str.parse::<i32>());

    //         Ok(first_number * second_number)
    //     }

    //     fn print(result: Result<i32, ParseIntError>) {
    //         match result {
    //             Ok(n) => println!("n is {}", n),
    //             Err(e) => println!("Error: {}", e),
    //         }
    //     }
    //     print(multiply("10", "2"));
    //     print(multiply("t", "2"));
    // }
}

///
/// 前面的示例中使用了很多简便的方法，也讲解了两个很重要的类型 `Option` 和 `Result`，
/// `Option` 可以和 `Option` 交互，`Result` 可以和 `Result` 交互。
///
/// 但是有些情况下我们可能需要 `Option` 和 `Result` 之间进行交互，或者 `Result<T, Error1>`
/// 和 `Result<T, Result2>` 进行交互，在这种情况下我们希望以一种可组合且易于维护的方式来处理不同的错误类型。
///
/// 下面这个例子中就出现了两种不同的错误类型。
///
fn multiple_error_types() {
    fn double_first(vec: Vec<&str>) -> i32 {
        let first = vec.first().unwrap(); // 错误1
        2 * first.parse::<i32>().unwrap() // 错误2
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // 错误 1: 输入的数据是空的

    println!("The first doubled is {}", double_first(strings));
    // 错误 2: 第一个元素不能被转换成数字
}

///
/// 最简单的方法是直接把两个对象嵌套起来使用
///
fn pulling_results_out_of_options() {
    // `Option<Result<T, E>>` 包装
    {
        use std::num::ParseIntError;

        fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
            // 使用 `map` 方法会把闭包的返回值包装成 `Option<T>`，
            // 闭包中返回的是 `Result` 类型
            vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));

        println!("The first doubled is {:?}", double_first(empty));
        // 错误 1: 输入是空的

        println!("The first doubled is {:?}", double_first(strings));
        // 错误 2: 第一个元素不能转换为数字
    }

    // `Result<Option<T>, E>` 包装
    {
        use std::num::ParseIntError;

        fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
            // `map` 方法包装成 `Option<T>`，闭包中返回 `Result<T>`
            let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

            // 使用 `Option` 的 `map_or` 方法解包并做反向包装
            opt.map_or(Ok(None), |r| r.map(Some))
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));
        println!("The first doubled is {:?}", double_first(empty));
        println!("The first doubled is {:?}", double_first(strings));
    }
}

///
/// 自定义错误类型
///
/// 大多数时候都会选择定义一个错误的类型，这个错误类型包含所有可能出现的错误，
/// 通常情况下定义的错误约定如下
///
/// - 在同一个类型下定义多种可能出现的错误
/// - 能给用户提供一个明确的错误信息
/// - 可以非常容易的通过 `if let` 或者 `match` 语句进行判断
///     - 推荐 `Err(EmptyVec)`
///     - 不推荐 `Err("Please use a vector with at least one element".to_owned())`
/// - 可以储存明确的错误信息
///     - 推荐 `Err(BadChar(c, position))`
///     - 不推荐 `Err("+ cannot be used here".to_owned())`
/// - 可以组合使用
///
fn defining_an_error_type() {
    use std::fmt;

    // 定义一个类型别名
    type Result<T> = std::result::Result<T, DoubleError>;

    // 定义错误类型
    #[derive(Debug, Clone)]
    struct DoubleError;

    // 对以错误类型实现格式化特性，让使用方可以直接把错误以文字形式展示给用户。
    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            // `ok_or` 方法转换 `Option` 为 `Result` 修改错误为我们定义的类型
            .ok_or(DoubleError)
            .and_then(|s| {
                s.parse::<i32>()
                    // 把错误转换成自己的类型
                    .map_err(|_| DoubleError)
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

///
/// 错误装箱
///
/// 当使用了自定义错误的时候会产生比较多额外的转化代码，可以通过 `Box` 来快速的包装错误转换逻辑，
/// 这里有一个缺点就是当使用 `Box` 对象就代表我们的对象在编译阶段是无法确定大小的。
///
/// 标准库中提供了一个特性是 `std::error:Error` 我们实现这个特性就可以让自定义错误可以自动装箱
///
fn boxing_errors() {
    use std::error;
    use std::fmt;

    // 修改类型别名 `Box<error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    // 实现 Display 特性，因为 `std::error::Error` 特性要求必须要实现这个特性
    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    // 实现标准错误特性来提供自动装箱的能力
    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            // `ok_or` 方法转换 `Option` 为 `Result`
            .ok_or_else(|| EmptyVec.into()) // 使用自动装箱提供的转换能力
            .and_then(|s| {
                s.parse::<i32>()
                    // 因为 `ParseIntError` 同样也实现了标准错误 `std::error::Error` 所以这里实际上是为了把 `ParseIntError` 转换成 `dyn Error`
                    .map_err(|e| e.into())
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

///
/// `?` 操作符的其他用法
///
/// `?` 操作符只能在 `Result` 对象上使用，那么对于 `Option` 对象我们可以使用
/// `ok_or`、`ok_or_else` 两个方法把 `Option` 转换成 `Result` 对象。
///
fn other_uses_of_question_mark() {
    use std::error;
    use std::fmt;

    // 修改错误类型为 `Box<dyn error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    // 这里使用 `?` 对 `Result` 进行数据提取，提取失败的话直接返回原始错误。
    // 进行修改后 代码简洁了很多，也很清晰。
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        // `first()` 方法返回的是 `Option`，通过 `ok_or` 方法转换成 `Result`
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

///
/// 最终完整的错误解决方案应该是把其他类型的错误封装到自己定义的错误中
///
fn wrapping_errors() {
    use std::error;
    use std::error::Error;
    use std::fmt;
    use std::num::ParseIntError;

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // 自定义一个错误类型，内部封装了 `ParseIntError` 错误
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                // 实现包装错误的特殊信息提示
                DoubleError::Parse(..) => {
                    write!(f, "the provided string could not be parsed as int")
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                // The cause is the underlying implementation error type. Is implicitly
                // cast to the trait object `&error::Error`. This works because the
                // underlying type already implements the `Error` trait.
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    // 实现从 `ParseIntError` 类型转换到自定义的 `DoubleError` 类型。
    // 当 `?` 操作符需要转换 `ParseIntError` 到 `DoubleError` 的时候会自动调用该函数
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        // 这里如果出现错误 `?` 会自动调用上面实现的转换方法。
        let parsed = first.parse::<i32>()?;

        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("  Caused by: {}", source);
                }
            }
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

///
/// 在迭代器中使用 `Result`
///
fn iterating_over_results() {
    // `Iter::map` 的闭包返回值可能是错误
    {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
        println!("Results: {:?}", numbers);
    }
    // 使用 `Iter::filter_map` 来过滤掉失败的结果
    {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        println!("Results: {:?}", numbers);
    }

    // 使用 `map_err` 和 `filter_map` 来分别保存成功和失败
    {
        let strings = vec!["42", "tofu", "93", "999", "18"];
        let mut errors = vec![];
        let numbers: Vec<_> = strings
            .into_iter()
            .map(|s| s.parse::<u8>())
            .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
            .collect();
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }

    // 如果发现有失败的情况则直接终止转换，并返回错误
    // 这里因为 `Result` 实现了 `FromIterator` 特性，
    // 所以可以自动转换 `Vec<Result<T, E>>` 类型为 `Result<Vec<T>, E>`
    // 但是一旦转换过程中出现 `Result:Err` 则迭代器就会直接终止，不再尝试后续转换。
    {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
        println!("Results: {:?}", numbers);
    }

    // 使用 `partition` 方法来自动对数据进行分类。
    {
        let strings = vec!["tofu", "93", "18"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            // partition 接收一个闭包，该闭包返回一个 `bool` 值，然后对两种返回值自动进行分类，并且返回分类好的元组
            .partition(Result::is_ok);
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }

    // 经过上面的分类发现结果还是包装在了 `Result` 中，
    // 想要去掉层包装只能再多写两行模版代码来去掉不同类型的包装
    {
        let strings = vec!["tofu", "93", "18"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);

        // 使用 `Result::unwrap` 方法来去掉 `Ok` 的包装
        let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();

        // 使用 `Result::unwrap_err` 方法来去掉 `Err` 的包装
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }
}

fn main() {
    // 明确调用 `panic!` 主动退出。
    // panic_();

    // 使用宏来定义不同的错误行为
    // abort_and_unwind();

    // 使用 `Option` 来处理错误。
    // option_and_unwrap();

    // `Option` 相关的使用方法
    {
        // 使用 `?` 操作符来快捷获取数据。
        unpacking_options_with_question_mark();

        // `map` 方法只处理有值的情况，并对闭包返回值进行 `Option<T>` 包装。
        combinators_map();

        // `and_then` 方法只处理有值的情况，不对返回值进行包装，但是要求闭包必须返回一个 `Option<T>` 的包装值。
        combinators_and_then();

        // 常用的其他方法 `or` `or_else` `get_or_insert` `get_or_insert_with`
        unpacking_options_and_default();
    }

    // `Result` 相关的使用方法
    {
        // `Result` 对象的基础介绍
        result();
        // 使用 `Result` 定义 `main` 函数的返回值
        using_result_in_main();
        // `map` 方法和 `Option` 中的方法一致
        map_for_result();
        // 为 `Result` 定义类型别名
        aliases_for_result();
        // 提前返回
        eraly_result();
        // `?` 操作符简化代码逻辑，以及操作符的前身 `try!` 宏
        intorducing_question_mark();
    }

    // 同时处理多种错误类型
    {
        // 基础示例
        multiple_error_types();
        // `Result` 和 `Option` 类型的互相嵌套使用
        pulling_results_out_of_options();
        // 自定义错误类型
        defining_an_error_type();
        // 错误类型的装箱，实现多种错误类型并存
        boxing_errors();
        // 使用 `?` 优化错误转换代码
        other_uses_of_question_mark();
        // 完整的自定义错误类型示例
        wrapping_errors();
    }

    // 迭代器中使用 `Result`
    iterating_over_results();
}
