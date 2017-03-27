fn main() {
    // 单行闭包是一个表达式
    {
        println!("#单行闭包是一个表达式");

        let plus_one = |params: u32| params + 1u32;
        let added_one = plus_one(1u32);

        println!("added_one: {:?}", added_one);
    }

    // 多行闭包使用花括号
    {
        // 一般不需要声明闭包参数和返回值的类型
        // 多行闭包使用花括号，通过返回值实现表达式
        println!("#多行闭包使用花括号");

        let plus_twice = |params| {
            let mut result: u32 = params;

            result += 2_u32;
            result += 3_u32;

            result
        };
        let added_twice = plus_twice(1u32);

        println!("added_twice: {:?}", added_twice);
    }

    // 闭包引用“不可变借用”
    {
        println!("#闭包引用“不可变借用");

        let base = 5_u32;

        // plus引用了base的"不可变借用"
        let plus = |params| params + base;
        let result = plus(5);

        // 虽然plus引用了base的"不可变借用"，但是5_32的所有权还是归base所有
        println!("base:{:?}", base);
        println!("result:{:?}", result);
    }

    // 闭包引用可变借用
    {
        println!("#闭包引用可变借用");

        let mut base = 5_u32;
        println!("base:{:?}", base); // 保证 base 被读过
        base = 10_u32; // 保证 base 被写过

        // plus引用了base的"可变借用"
        let plus = |params| params + base;
        let result = plus(5);

        // 虽然plus引用了base的"可变借用"，但是10_32的所有权还是归base所有
        println!("base:{:?}", base);
        println!("result:{:?}", result);

        // 因为plus引用了base的"可变借用"，所以 y 不能再取得base的"可变借用"了
        // let y = &mut base;
    }

    // 闭包引用可变借用，但是释放了闭包中的可变借用
    {
        println!("#闭包引用可变借用，但是释放了闭包中的可变借用");

        let mut base = 5_u32;
        println!("base:{:?}", base); // 保证 base 被读过
        base = 10_u32; // 保证 base 被写过

        {
            // plus引用了base的"可变借用"
            let plus = |params| params + base;
            let result = plus(5);

            // 虽然plus引用了base的"可变借用"，但是10_32的所有权还是归base所有
            println!("base:{:?}", base);
            println!("result:{:?}", result);
        }

        // 虽然plus引用了base的“可变借用”
        // 但是随着闭包 plus 离开其作用域，也释放了闭包中对 base 的“可变借用”
        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{:?}", borrow);
    }

    // 闭包中使用了实现Copy特性的基本类型，但不使用move
    {
        println!("闭包中使用了实现Copy特性的基本类型，但不使用move");

        let num = 5_u32;

        let closure = |params| params + num;
        let result = closure(1_u32);

        println!("num:{:?}", num);
        println!("result:{:?}", result);
    }

    // 闭包中使用了实现Copy特性的基本类型，而且使用move
    {
        println!("闭包中使用了实现Copy特性的基本类型，而且使用move");

        let num = 5_u32;

        let closure = move |params| params + num;
        let result = closure(1u32);

        println!("num:{:?}", num);
        println!("result:{:?}", result);
    }

    {
        let mut num = 5_u32;

        // A `mut` is required on `closure` because a `&mut` is stored inside.
        // Thus, calling the closure mutates the closure which requires a `mut`.
        let mut closure = |params| {
            num += params;
            println!("num:{:?}", num);
        };

        closure(1u32);
        closure(4u32);

        // println!("num:{:?}", num);
    }

    {
        let mut num = 5_u32;

        let mut closure = move |params| {
            num += params;
            println!("num:{:?}", num);
        };

        closure(1u32);
        closure(4u32);

        // println!("num:{:?}", num);
    }

    {
        let mut num = 5_u32;

        {
            let mut closure = move |params| {
                num += params;
                println!("num:{:?}", num);
            };

            closure(1u32);
            closure(4u32);
        }

        println!("num:{:?}", num);
    }

    {
        let mut string: String = String::from("abc");

        let mut closure = move |params: char| string.push(params);
        closure('d');

        // println!("string:{:?}", string);
    }

    {
        let mut string: String = String::from("abc");

        {
            let mut closure = |params: char| string.push(params);
            closure('d');
        }

        println!("string:{:?}", string);
    }
}