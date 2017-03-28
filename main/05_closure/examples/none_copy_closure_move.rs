fn main() {
    println!("#闭包引用了不具有Copy特性的非基本类型，如let base: String = String::from(\"abc\")");

    println!("\n##move闭包引用不可变借用");
    {
        let base: String = String::from("abc");

        // closure通过move获取了base的所有权
        let closure = move |params| (base.clone(), params);
        let result = closure('d');

        // println!("base:{}", base);
        println!("result:{:?}", result);
    }

    println!("\n##move闭包引用可变借用，不改变");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        let closure = move |params| (base.clone(), params);
        let result = closure('d');

        //  println!("base:{}", base);
        println!("result:{:?}", result);

        //  let borrow = &mut base;
    }

    println!("\n##move闭包引用可变借用，改变");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        let mut closure = move |params| base.push(params);
        let result = closure('d');

        // println!("base:{}", base);
        println!("result:{:?}", result);

        // let borrow = &mut base;
    }

    println!("\n##move闭包引用可变借用，不改变，释放了闭包中的可变借用");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        {
            let closure = move |params| (base.clone(), params);
            let result = closure('d');

            // println!("base:{}", base);
            println!("result:{:?}", result);
        }

        // println!("base:{}", base);

        // let borrow = &mut base;
    }

    println!("\n##move闭包引用可变借用，改变，释放了闭包中的可变借用");
    {
        let mut base: String = String::from("a");
        base = String::from(base.clone() + "bc");

        {
            let mut closure = move |params| base.push(params);
            let result = closure('d');

            // println!("base:{}", base);
            println!("result:{:?}", result);
        }

        // println!("base:{}", base);

        // let borrow = &mut base;
    }
}