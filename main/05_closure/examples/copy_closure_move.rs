fn main() {
    println!("# move闭包引用了具有Copy特性的基本类型，如let base = 5_u32");

    println!("\n## move闭包引用不可变借用");
    {
        let base = 5_u32;

        // closure先copy num
        // 然后move copyed numco
        // 最后获取copyed num的不可变引用为base
        let closure = move |params| params + base;
        let result = closure(5_u32);

        println!("base:{}", base);
        println!("result:{}", result);
    }

    println!("\n##move闭包引用可变借用，不改变");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let closure = move |params| params + base;
        let result = closure(5_u32);

        println!("base:{}", base);
        println!("result:{}", result);

        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{}", borrow);
    }

    println!("\n##move闭包引用可变借用，改变");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let mut closure = move |params| {
            base += params;
            base
        };
        let result = closure(5_u32);

        println!("base:{}", base);
        println!("result:{}", result);

        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{}", borrow);
    }

    println!("\n##move闭包引用可变借用，不改变，释放了闭包中的可变借用");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        {
            let closure = move |params| params + base;
            let result = closure(5_u32);

            println!("base:{}", base);
            println!("result:{}", result);
        }

        println!("base:{}", base);

        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{}", borrow);
    }

    println!("\n##move闭包引用可变借用，改变，释放了闭包中的可变借用");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        {
            let mut closure = move |params| {
                base += params;
                base
            };
            let result = closure(5_u32);

            println!("base:{}", base);
            println!("result:{}", result);
        }

        println!("base:{}", base);

        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{}", borrow);
    }
}