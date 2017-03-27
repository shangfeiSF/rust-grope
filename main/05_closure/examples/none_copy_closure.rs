fn main() {
    println!("#闭包中包含不具备Copy特性的非基本类型，如String");

    println!("\n##闭包引用不可变借用");
    {
        let base: String = String::from("abc");

        // closure引用了base的不可变借用
        let closure = |params| (&*base, params);
        let result = closure('d');

        // 虽然closure引用了base的不可变借用，但是String::from("abc")的所有权还是归base所有
        println!("base:{}", base);
        println!("result:{:?}", result);
    }

    println!("\n##闭包引用可变借用，不改变");
    {
        let mut base: String = String::from("abc");

        // 保证base被读过and被写过
        println!("base:{}", base);
        base = String::from("def");

        // closure引用了base的可变借用
        let closure = |params| (&*base, params);
        let result = closure('g');

        // 虽然closure引用了base的可变借用，但是没有改变base
        println!("base:{}", base);
        println!("result:{:?}", result);

        // 不过因为closure引用了base的可变借用，所以borrow不能再取得base的可变借用了
        // let borrow = &mut base;
    }

    println!("\n##闭包引用可变借用，改变");
    {
        let mut base: String = String::from("abc");

        // 保证base被读过and被写过
        println!("base:{}", base);
        base = String::from("def");

        // closure引用了base的可变借用
        let mut closure = |params| base.push(params);
        let result = closure('g');

        // closure引用了base的可变借用，但是改变了base
        // println!("base:{}", base);
        println!("result:{:?}", result);

        // 不过因为closure引用了base的可变借用，所以borrow不能再取得base的可变借用了
        // let borrow = &mut base;
    }

    println!("\n##闭包引用可变借用，不改变，释放了闭包中的可变借用");
    {
        let mut base: String = String::from("abc");

        // 保证base被读过and被写过
        println!("base:{}", base);
        base = String::from("def");

        {
            // 闭包closure所在的作用域
            // closure引用了base的可变借用
            let closure = |params| (&*base, params);
            let result = closure('g');

            // 虽然closure引用了base的可变借用，但是没有改变base
            println!("base:{}", base);
            println!("result:{:?}", result);
        }

        // 虽然closure引用了base的可变借用
        // 但是随着闭包closure离开其作用域，也释放了闭包中base的可变借用
        // 所以borrow可以取得base的可变借用了
        let borrow = &mut base;
        *borrow = String::from("xyz");

        println!("borrow:{}", borrow);
    }

    println!("\n##闭包引用可变借用，改变，释放了闭包中的可变借用");
    {
        let mut base: String = String::from("abc");

        // 保证base被读过and被写过
        println!("base:{}", base);
        base = String::from("def");

        {
            // 闭包closure所在的作用域
            // closure引用了base的可变借用
            let mut closure = |params| base.push(params);
            let result = closure('g');

            // closure引用了base的可变借用，但是改变了base
            // println!("base:{}", base);
            println!("result:{:?}", result);
        }

        // 虽然closure引用了base的可变借用
        // 但是随着闭包closure离开其作用域，也释放了闭包中base的可变借用
        // 所以borrow可以取得base的可变借用了
        let borrow = &mut base;
        *borrow = String::from("xyz");

        println!("borrow:{}", borrow);
    }
}