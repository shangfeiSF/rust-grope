fn main() {
    {
        fn call_with_closure<F>(some_closure: F) -> i32
            where F: Fn(i32) -> i32 // 约束条件F: Fn(i32) -> i32，是针对闭包设计的专门的语法
        {
            some_closure(1)
        }
        // 让闭包作为一个参数，传递到函数中
        let answer = call_with_closure(|x| x + 2);
        println!("answer = {}", answer);
    }


    {
        let closure_1 = |x| x * 2;
        let closure_2 = |x| x * 3;
        fn normal(x: i32) -> i32 {
            x * 4
        };

        // 向函数中传递闭包，有两种方式：

        // （1）泛型的方式
        // 这种方式会为不同的闭包参数类型生成不同版本的函数，实现静态分派

        // 对于每个不同类型的参数，编译器将会生成不同版本的函数。
        fn static_dispatch<F>(closure: &F)
            where F: Fn(i32) -> i32
        {
            println!("static dispatch `closure(10)` = {}", closure(10));
        }

        static_dispatch(&closure_1);
        static_dispatch(&closure_2);
        static_dispatch(&normal); // 普通`fn`函数也实现了`Fn trait`，它可以与此参数类型匹配，但是`fn`不可以捕获外部变量

        // （2）通过trait object的方式
        // 这种方式会将闭包box进入堆内存中，向函数传递一个胖指针，实现运行期动态分派

        // 这里是 `trait object`  ，`Box<Fn(i32)->i32>`也算`trait object`
        fn dynamic_dispatch(closure: &Fn(i32) -> i32) {
            println!("dynamic dispatch `closure(10)` = {}", closure(10));
        }

        dynamic_dispatch(&closure_1);
        dynamic_dispatch(&closure_2);
        dynamic_dispatch(&normal);
    }
}
