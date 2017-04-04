fn main() {
    println!("\n## let base: String = String::from(\"abc\"); 闭包实现: Fn trait, 匿名结构体捕获: &base");
    {
        let base: String = String::from("abc");

        // 闭包实现: Fn trait, 匿名结构体捕获: &base
        let closure = move |params| {
            (&base, params);
            params
        };
        let call_closure_1 = closure('d');
        let call_closure_2 = closure('e');

        println!("call_closure_1 = {:?}", call_closure_1);
        println!("call_closure_2 = {:?}", call_closure_2);

        println!("base = {:?}", base);
        let borrow_read_1 = &base;
        println!("borrow_read_1 = {:?}", borrow_read_1);
        let borrow_read_2 = &base;
        println!("borrow_read_2 = {:?}", borrow_read_2);

        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: Fn trait, 匿名结构体捕获: &base");
    {
        let mut base: String = String::from("abc");

        // 闭包实现: Fn trait, 匿名结构体捕获: &base
        let closure = move |params| {
            (&base, params);
            params
        };
        let call_closure_1 = closure('d');
        let call_closure_2 = closure('e');

        println!("call_closure_1 = {:?}", call_closure_1);
        println!("call_closure_2 = {:?}", call_closure_2);

        println!("base = {:?}", base);
        let borrow_read_1 = &base;
        println!("borrow_read_1 = {:?}", borrow_read_1);
        let borrow_read_2 = &base;
        println!("borrow_read_2 = {:?}", borrow_read_2);

        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: Fn trait，匿名结构体捕获: &base，但是释放了闭包作用域，之后“读or写”");
    {
        let mut base: String = String::from("abc");

        {
            // 闭包实现: Fn trait, 匿名结构体捕获: &base
            let closure = move |params| {
                (&base, params);
                params
            };
            let call_closure_1 = closure('d');
            let call_closure_2 = closure('e');

            println!("call_closure_1 = {:?}", call_closure_1);
            println!("call_closure_2 = {:?}", call_closure_2);

            println!("base = {:?}", base);
            let borrow_read_1 = &base;
            println!("borrow_read_1 = {:?}", borrow_read_1);
            let borrow_read_2 = &base;
            println!("borrow_read_2 = {:?}", borrow_read_2);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用，不影响之后的“写”
        let borrow_write = &mut base;
        *borrow_write = String::from("xyz");
        println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: FnMut trait, 匿名结构体捕获: &mut base");
    {
        let mut base: String = String::from("abc");

        // 闭包实现: FnMut trait, 匿名结构体捕获: &mut base
        let mut closure = move |params| {
            base.push(params);
            params
        };
        let call_closure_1 = closure('d');
        let call_closure_2 = closure('e');

        println!("call_closure_1 = {:?}", call_closure_1);
        println!("call_closure_2 = {:?}", call_closure_2);

        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: FnMut trait，匿名结构体捕获: &mut base，但是释放了闭包作用域，之后“读or写”");
    {
        let mut base: String = String::from("abc");

        {
            // 闭包实现: FnMut trait, 匿名结构体捕获: &mut base
            let mut closure = move |params| {
                base.push(params);
                params
            };
            let call_closure_1 = closure('d');
            let call_closure_2 = closure('e');

            println!("call_closure_1 = {:?}", call_closure_1);
            println!("call_closure_2 = {:?}", call_closure_2);

            // println!("base = {}", base);
            // let borrow_read_1 = &base;
            // println!("borrow_read_1 = {}", borrow_read_1);
            // let borrow_read_2 = &base;
            // println!("borrow_read_2 = {}", borrow_read_2);

            // let borrow_write = &mut base;
            // *borrow_write = String::from("xyz");
            // println!("borrow_write = {:?}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用，不影响之后的“写”
        let borrow_write = &mut base;
        *borrow_write = String::from("xyz");
        println!("borrow_write = {:?}", borrow_write);
    }
}