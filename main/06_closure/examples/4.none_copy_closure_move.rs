fn main() {
    println!("\n## let base: String = String::from(\"abc\"); 闭包实现: Fn trait, move强制匿名结构体捕获: base");
    {
        let base: String = String::from("abc");

        // 闭包实现: Fn trait, move强制匿名结构体捕获: base
        let closure = move |params| {
            (&base, params);
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

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: Fn trait, move强制匿名结构体捕获: base");
    {
        let mut base: String = String::from("abc");

        // 闭包实现: Fn trait, move强制匿名结构体捕获: base
        let closure = move |params| {
            (&base, params);
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

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: Fn trait，move强制匿名结构体捕获: base，释放了闭包作用域也不行");
    {
        let mut base: String = String::from("abc");

        {
            // 闭包实现: Fn trait, move强制匿名结构体捕获: base
            let closure = move |params| {
                (&base, params);
                params
            };
            let call_closure_1 = closure('d');
            let call_closure_2 = closure('e');

            // println!("call_closure_1 = {:?}", call_closure_1);
            // println!("call_closure_2 = {:?}", call_closure_2);

            // println!("base = {:?}", base);
            // let borrow_read_1 = &base;
            // println!("borrow_read_1 = {:?}", borrow_read_1);
            // let borrow_read_2 = &base;
            // println!("borrow_read_2 = {:?}", borrow_read_2);
        }

        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: FnMut trait, move强制匿名结构体捕获: base");
    {
        let mut base: String = String::from("abc");

        // 闭包实现: FnMut trait, move强制匿名结构体捕获: base
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

    println!("\n## let mut base: String = String::from(\"abc\"); 闭包实现: FnMut trait，move强制匿名结构体捕获: base，释放了闭包作用域也不行");
    {
        let mut base: String = String::from("abc");

        {
            // 闭包实现: FnMut trait, move强制匿名结构体捕获: base
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

        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }
}