fn main() {
    {
        // 如果希望一个闭包作为函数的返回值，那么就不能使用泛型的方式
        // 因为泛型类型不在参数列表中出现，而仅仅在返回类型中出现的话
        // 会要求在调用的时候显式指定类型，编译器才能完成类型推导
        // 可是调用方根本无法指定具体类型，因为闭包类型是匿名类型，无法显式指定

        // `fn test_error<F>() -> F`这样的写法是编译不过的

        // fn test_error<F>() -> F
        //     where F: Fn(i32) -> i32
        // {
        //     return |num| num * 2;
        // }
        // let closure_error = test_error();

        // 目前提出了一份 RFC，希望加入一个新的语法`fn test_correct() -> impl Fn(i32)->i32`
        // 这样的语法糖可以简化代码，但为达成一致
        // 目前来说，唯一的方式就是把闭包装箱（Box）进入堆内存中，使用Box< Fn(i32)->i32 >类型返回

        fn test_correct() -> Box<Fn(i32) -> i32> {
            let c = |num: i32| num * 2;
            Box::new(c)
        }

        let closure_correct = test_correct();
        let result = closure_correct(2);
        println!("result = {}", result);
    }
}