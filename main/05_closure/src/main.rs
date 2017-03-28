fn main() {
    println!("#单行闭包是一个表达式");
    {
        let base = 5_u32;

        let closure = |params: u32| params + base;
        let result = closure(5_u32);

        println!("base: {}", base);
        println!("result: {}", result);
    }

    println!("#多行闭包使用花括号，通过返回值实现表达式");
    {
        let mut base = 5_u32;

        let mut closure = |params| {
            // 不需要声明参数和返回值的类型
            base += params;
            base
        };
        let result = closure(5_u32);

        // println!("base: {}", base);
        println!("result: {}", result);
    }
}