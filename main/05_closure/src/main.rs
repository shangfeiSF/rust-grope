fn main() {
    println!("#单行闭包是一个表达式");
    {
        let plus_one = |params: u32| params + 1u32;
        let added_one = plus_one(5u32);

        println!("added_one: {}", added_one);
    }

    println!("#多行闭包使用花括号，通过返回值实现表达式");
    {
        // 不需要声明闭包参数和返回值的类型
        let plus_twice = |params| {
            let mut result: u32 = params;

            result += 2_u32;
            result += 3_u32;

            result
        };
        let added_twice = plus_twice(1u32);

        println!("added_twice: {}", added_twice);
    }
}