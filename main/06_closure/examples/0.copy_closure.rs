fn main() {
    println!("\n## let base, 闭包实现: Fn trait, 匿名结构体捕获: &base");
    {
        let base = 5_u32;

        // 闭包实现: Fn trait, 匿名结构体捕获: &base
        let closure = |params| params + &base;
        let call_closure_1 = closure(10_u32);
        let call_closure_2 = closure(20_u32);

        println!("call_closure_1 = {}", call_closure_1);
        println!("call_closure_2 = {}", call_closure_2);

        // 可读
        println!("base = {}", base);
        let borrow_read_1 = &base;
        println!("borrow_read_1 = {}", borrow_read_1);
        let borrow_read_2 = &base;
        println!("borrow_read_2 = {}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = 30_u32;
        // println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base, 闭包实现: Fn trait, 匿名结构体捕获: &base");
    {
        let mut base = 5_u32;

        // 闭包实现: Fn trait，匿名结构体捕获: &base
        let closure = |params| params + &base;
        let call_closure_1 = closure(10_u32);
        let call_closure_2 = closure(20_u32);

        println!("call_closure_1 = {}", call_closure_1);
        println!("call_closure_2 = {}", call_closure_2);

        // 可读
        // println!("base = {}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = 30_u32;
        // println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base, 闭包实现: Fn trait, 匿名结构体捕获: &base, 释放闭包作用域");
    {
        let mut base = 5_u32;

        {
            // 闭包实现: Fn trait，匿名结构体捕获: &base
            let closure = |params| params + &base;
            let call_closure_1 = closure(10_u32);
            let call_closure_2 = closure(20_u32);

            println!("call_closure_1 = {}", call_closure_1);
            println!("call_closure_2 = {}", call_closure_2);

            // 可读
            println!("base = {}", base);
            let borrow_read_1 = &base;
            println!("borrow_read_1 = {}", borrow_read_1);
            let borrow_read_2 = &base;
            println!("borrow_read_2 = {}", borrow_read_2);

            // 不可写
            // let borrow_write = &mut base;
            // *borrow_write = 30_u32;
            // println!("borrow_write = {}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也释放了匿名结构体捕获的引用

        // 可读
        // println!("base = {}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {}", borrow_read_2);

        // 可写
        let borrow_write = &mut base;
        *borrow_write = 40_u32;
        println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base, 闭包实现: FnMut trait, 匿名结构体捕获: &mut base");
    {
        let mut base = 5_u32;

        // 闭包实现: FnMut trait，匿名结构体捕获: &mut base
        let mut closure = |params| {
            // A `mut` is required on `closure` because a `&mut` is stored inside.
            // Thus, calling the closure mutates the closure which requires a `mut`.
            base += params;
            base
        };
        let call_closure_1 = closure(10_u32);
        let call_closure_2 = closure(20_u32);

        println!("call_closure_1 = {}", call_closure_1);
        println!("call_closure_2 = {}", call_closure_2);

        // 不可读
        // println!("base = {}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = 30_u32;
        // println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base,  闭包实现: FnMut trait, 匿名结构体捕获: &mut base, 释放闭包作用域");
    {
        let mut base = 5_u32;

        {
            // 闭包实现: FnMut trait，匿名结构体捕获: &mut base
            let mut closure = |params| {
                // A `mut` is required on `closure` because a `&mut` is stored inside.
                // Thus, calling the closure mutates the closure which requires a `mut`.
                base += params;
                base
            };
            let call_closure_1 = closure(10_u32);
            let call_closure_2 = closure(20_u32);

            println!("call_closure_1 = {}", call_closure_1);
            println!("call_closure_2 = {}", call_closure_2);

            // 不可读
            // println!("base = {}", base);
            // let borrow_read_1 = &base;
            // println!("borrow_read_1 = {}", borrow_read_1);
            // let borrow_read_2 = &base;
            // println!("borrow_read_2 = {}", borrow_read_2);

            // 不可写
            // let borrow_write = &mut base;
            // *borrow_write = 30_u32;
            // println!("borrow_write = {}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也释放了匿名结构体捕获的引用

        // 可读
        // println!("base = {}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {}", borrow_read_2);

        // 可写
        let borrow_write = &mut base;
        *borrow_write = 40_u32;
        println!("borrow_write = {}", borrow_write);
    }

    println!("\n## copy trait的影响");
    {
        let mut base = 5_u32;

        // 因为base具备copy trait，所以闭包实现的不是FnOnce，而是Fn
        // 因为base具备copy trait，所以匿名结构体捕获的不是base，而是&base
        let closure = |params| {
            drop(base);
            params
        };

        let call_closure_1 = closure(10_u32);
        let call_closure_2 = closure(20_u32);

        println!("call_closure_1 = {}", call_closure_1);
        println!("call_closure_2 = {}", call_closure_2);

        // 可读
        println!("base = {}", base);
        let borrow_read_1 = &base;
        println!("borrow_read_1 = {}", borrow_read_1);
        let borrow_read_2 = &base;
        println!("borrow_read_2 = {}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = 30_u32;
        // println!("borrow_write = {}", borrow_write);
    }
}