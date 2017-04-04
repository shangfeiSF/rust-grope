fn main() {
    println!("\n## let base,  闭包实现: Fn trait, move强制匿名结构体捕获: base");
    {
        let base = 5_u32;

        // 闭包实现: Fn trait, move强制匿名结构体捕获: base
        let closure = move |params| params + base;
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

    println!("\n## let mut base,  闭包实现: Fn trait，move强制匿名结构体捕获: base");
    {
        let mut base = 5_u32;

        // 闭包实现: Fn trait，move强制匿名结构体捕获: base
        let closure = move |params| params + base;
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

        // 可写
        let borrow_write = &mut base;
        *borrow_write = 30_u32;
        println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base, 闭包实现: Fn trait, move强制匿名结构体捕获: base, 释放闭包作用域之后“读or写”");
    {
        let mut base = 5_u32;

        {
            // 闭包实现: Fn trait，move move强制匿名结构体捕获: base
            let closure = move |params| params + base;
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

            // 可写
            let borrow_write = &mut base;
            *borrow_write = 30_u32;
            println!("borrow_write = {}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用，不影响之后的“写”

        // 可读
        // println!("base = {}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {}", borrow_read_2);

        // 可写
        let borrow_write = &mut base;
        *borrow_write = 30_u32;
        println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base = 5_u32; 闭包实现: FnMut trait，move强制匿名结构体捕获: base");
    {
        let mut base = 5_u32;

        // 闭包实现: FnMut trait，move强制匿名结构体捕获: base
        let mut closure = move |params| {
            // A `mut` is required on `closure` because a `&mut` is stored inside.
            // Thus, calling the closure mutates the closure which requires a `mut`.
            base += params;
            base
        };
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

        // 可写
        let borrow_write = &mut base;
        *borrow_write = 30_u32;
        println!("borrow_write = {}", borrow_write);
    }

    println!("\n## let mut base, 闭包实现: FnMut trait, move强制匿名结构体捕获: base, 释放闭包作用域之后“读or写”");
    {
        let mut base = 5_u32;
        {
            // 闭包实现: FnMut trait，move强制匿名结构体捕获: base
            let mut closure = move |params| {
                // A `mut` is required on `closure` because a `&mut` is stored inside.
                // Thus, calling the closure mutates the closure which requires a `mut`.
                base += params;
                base
            };
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

            // 可写
            let borrow_write = &mut base;
            *borrow_write = 30_u32;
            println!("borrow_write = {}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用，不影响之后的“写”

        // 可读
        // println!("base = {}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {}", borrow_read_2);

        // 可写
        let borrow_write = &mut base;
        *borrow_write = 30_u32;
        println!("borrow_write = {}", borrow_write);
    }
}