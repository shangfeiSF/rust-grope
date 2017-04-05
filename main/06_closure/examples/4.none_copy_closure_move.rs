fn main() {
    println!("\n## let base，闭包实现: Fn trait, move强制匿名结构体捕获: base");
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

        // 不可读
        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base，闭包实现: Fn trait, move强制匿名结构体捕获: base");
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

        // 不可读
        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base，闭包实现: Fn trait，move强制匿名结构体捕获: base，释放了闭包作用域也不行");
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

            println!("call_closure_1 = {:?}", call_closure_1);
            println!("call_closure_2 = {:?}", call_closure_2);

            // 不可读
            // println!("base = {:?}", base);
            // let borrow_read_1 = &base;
            // println!("borrow_read_1 = {:?}", borrow_read_1);
            // let borrow_read_2 = &base;
            // println!("borrow_read_2 = {:?}", borrow_read_2);

            // 不可写
            // let borrow_write = &mut base;
            // *borrow_write = String::from("xyz");
            // println!("borrow_write = {:?}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也无法访问move到闭包中的base

        // 不可读
        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base，闭包实现: FnMut trait, move强制匿名结构体捕获: base");
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

        // 不可读
        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## let mut base，闭包实现: FnMut trait，move强制匿名结构体捕获: base，释放了闭包作用域也不行");
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

            // 不可读
            // println!("base = {:?}", base);
            // let borrow_read_1 = &base;
            // println!("borrow_read_1 = {:?}", borrow_read_1);
            // let borrow_read_2 = &base;
            // println!("borrow_read_2 = {:?}", borrow_read_2);

            // 不可写
            // let borrow_write = &mut base;
            // *borrow_write = String::from("xyz");
            // println!("borrow_write = {:?}", borrow_write);
        }

        // 随着闭包closure离开其作用域，也无法访问move到闭包中的base

        // 不可读
        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## copy trait的影响");
    {
        let mut base: String = String::from("abc");

        // 因为base不具备copy trait，闭包实现: FnOnce，匿名结构体捕获: base
        let closure = move |params| {
            drop(base);
            params
        };

        let call_closure_1 = closure(10_u32);
        // let call_closure_2 = closure(20_u32);

        println!("call_closure_1 = {}", call_closure_1);
        // println!("call_closure_2 = {}", call_closure_2);

        // 不可读
        // println!("base = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = String::from("xyz");
        // println!("borrow_write = {:?}", borrow_write);
    }

    println!("\n## move关键字，只是影响了环境变量被捕获的方式");
    {
        let mut base: Vec<i32> = vec![1, 2, 3];

        // closure实现Fn，move强制捕获base
        let closure = move || for i in &base {
            print!("{}, ", i);
        };
        closure();
        closure();

        // 不可读
        // println!("\nbase = {:?}", base);
        // let borrow_read_1 = &base;
        // println!("borrow_read_1 = {:?}", borrow_read_1);
        // let borrow_read_2 = &base;
        // println!("borrow_read_2 = {:?}", borrow_read_2);

        // 不可写
        // let borrow_write = &mut base;
        // *borrow_write = vec![10, 20, 30];
        // println!("borrow_write = {:?}", borrow_write);
    }
}