//!
//! 关联项是指与各种类型的项目有关的一组规则，
//! 他是泛型特征的扩展，允许泛型特征在内部自定义新的项。
//!
//! 其中有一种成为关联类型，也就是实现特征的时候需要明确指定特征上定义的项的类型。
//!

fn the_problem() {
    struct Container(i32, i32);

    // 该特性明确要求需要传递 `<A, B>` 两个泛型类型，
    // 还有两个方法可以获取 `first`，`last` 两个值。
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // 明确指明需要泛型 `A`， `B`
        fn first(&self) -> i32; // 不需要泛型 `A` 或 `B`.
        fn last(&self) -> i32; // 不需要泛型 `A` 或 `B`.
    }

    // 明确声明要实现的类型是 `Contains<i32, i32>`，这样就代表泛型特性的 `<A, B>` 就是 `<i32, i32>`
    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    // 问题是这里的三个泛型的之间的关系，
    // 这里 C 被约束为 Contains<A, B>, 而我们写约束的时候很麻烦。
    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn associated_types() {
    struct Container(i32, i32);

    // 还是相同的定义，只不过这里不再使用泛型。
    trait Contains {
        // 定义两个通用的关联类型 `A`，`B`，这个时候还不知道关联类型的具体类型是什么，
        // 具体可能是什么类型是由实现该特性的地方定义的。
        type A;
        type B;

        // 这里 `Self::A` 中的 `Self` 实际上是指的 `Contains`，也就是当前的特性。
        // 到实际的实现的时候这个 `Self` 会被替换成实现该特性的实际类型。
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // 明确指定特性中定义的关联类型的具体类型。
        type A = i32;
        type B = i32;

        // 这里的 `number_1: &i32, number_2: &i32` 可以被替换成 `number_1: &Self::A, number_2: &Self::B`,
        // 因为 `Self` 等同于 `Container`
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    // 这里进行约束的时候就不需要额外的声明 `Contains` 特性所需要的两个泛型类型了。
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn main() {
    the_problem();
    associated_types();
}
