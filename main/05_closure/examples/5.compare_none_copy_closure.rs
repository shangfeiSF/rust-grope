fn main() {
    println!("#闭包引用了不具有Copy特性的非基本类型，如let base: String = String::from(\"abc\")");

    println!("\n## 闭包引用<不可变借用>，再let borrow = &base;");
    println!("******* normal *******");
    {
        let base: String = String::from("abc");

        // closure引用了base的不可变借用
        let closure = |params| (&base, params); // <不可变借用>
        let result = closure('d');

        println!("result:{:?}", result);

        println!("base:{:?}", base);

        let borrow = &base;
        println!("borrow:{:?}", borrow);

        let borrow_another = &base;
        println!("borrow_another:{:?}", borrow_another);
    }
    println!("******* move *******");
    {
        let base: String = String::from("abc");

        // closure move后获取了base的所有权
        let closure = move |params| (base, params); // <不可变借用>
        let result = closure('d');

        println!("result:{:?}", result);

        // println!("base:{:?}", base);

        // let borrow = &base;
        // println!("borrow:{:?}", borrow);

        // let borrow_another = &base;
        // println!("borrow_another:{:?}", borrow_another);
    }

    println!("\n## 闭包引用<不可变借用>，再let borrow = &mut base;");
    println!("******* normal *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        let closure = |params| (&base, params); // <不可变借用>
        let result = closure('d');

        println!("result:{:?}", result);

        println!("base:{:?}", base);

        // let borrow = &mut base;
        // *borrow = String::from("xyz");
        // println!("borrow:{:?}", borrow);
    }
    println!("******* move *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        let closure = move |params| (base, params); // <不可变借用>
        let result = closure('d');

        println!("result:{:?}", result);

        // println!("base:{:?}", base);

        // let borrow = &mut base;
        // *borrow = String::from("xyz");
        // println!("borrow:{:?}", borrow);
    }

    println!("\n## 闭包引用<可变借用>");
    println!("******* normal *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        // A `mut` is required on `closure` because a `&mut` is stored inside.
        // Thus, calling the closure mutates the closure which requires a `mut`.
        let mut closure = |params| base.push(params); // <可变借用>
        let result = closure('d');

        println!("result:{:?}", result);

        // println!("base:{:?}", base);

        // let borrow = &mut base;
        // *borrow = String::from("xyz");
        // println!("borrow:{:?}", borrow);
    }
    println!("******* move *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        // A `mut` is required on `closure` because a `&mut` is stored inside.
        // Thus, calling the closure mutates the closure which requires a `mut`.
        let mut closure = move |params| base.push(params); // <可变借用>
        let result = closure('d');

        println!("result:{:?}", result);

        // println!("base:{:?}", base);

        // let borrow = &mut base;
        // *borrow = String::from("xyz");
        // println!("borrow:{:?}", borrow);
    }

    println!("\n## 闭包引用<不可变借用>，再let borrow = & base;，<释放了闭包>");
    println!("******* normal *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        {
            let closure = |params| (&base, params);
            let result = closure('d');

            println!("result:{:?}", result);

            println!("base:{:?}", base);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用
        let borrow = &mut base;
        *borrow = String::from("xyz");
        println!("borrow:{:?}", borrow);
    }
    println!("******* move *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        {
            let closure = move |params| (base, params);
            let result = closure('d');

            println!("result:{:?}", result);

            // println!("base:{:?}", base);
        }

        // base的所有权已经被闭包closure获取
        // let borrow = &mut base;
        // *borrow = String::from("xyz");
        // println!("borrow:{:?}", borrow);
    }

    println!("\n## 闭包引用<可变借用>，再let borrow = &mut base;，<释放了闭包>");
    println!("******* normal *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        {
            let mut closure = |params| base.push(params); // <可变借用>
            let result = closure('d');

            println!("result:{:?}", result);

            // println!("base:{:?}", base);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用
        let borrow = &mut base;
        *borrow = String::from("xyz");
        println!("borrow:{:?}", borrow);
    }
    println!("******* move *******");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        {
            let mut closure = move |params| base.push(params); // <可变借用>
            let result = closure('d');

            println!("result:{:?}", result);

            // println!("base:{:?}", base);
        }

        // base的所有权已经被闭包closure获取
        // let borrow = &mut base;
        // *borrow = String::from("xyz");
        // println!("borrow:{:?}", borrow);
    }
}