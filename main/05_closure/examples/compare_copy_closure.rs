fn main() {
    println!("# 闭包引用了具有Copy特性的基本类型，如let base = 5_u32");

    println!("\n## 闭包引用不可变借用");
    println!("******* normal *******");
    {
        let base = 5_u32;

        // closure先copy num
        // 然后获取copyed num的不可变引用为base
        let closure = |params| params + base;
        let result = closure(5_u32);

        // 因为没有改变base
        println!("base:{}", base);
        println!("result:{}", result);
    }
    println!("******* move *******");
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

    println!("\n## 闭包引用可变借用，不改变");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let closure = |params| params + base;
        let result = closure(5_u32);

        // 因为没有改变base
        println!("base:{}", base);
        println!("result:{}", result);

        // 因为closure引用了base的可变借用，所以borrow不能再取得base的可变借用了
        // let borrow = &mut base;
    }
    println!("******* move *******");
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

    println!("\n## 闭包引用可变借用，改变");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        // A `mut` is required on `closure` because a `&mut` is stored inside.
        // Thus, calling the closure mutates the closure which requires a `mut`.
        let mut closure = |params| {
            base += params;
            base
        };
        let result = closure(5_u32);

        // 因为改变了base
        // println!("base:{}", base);
        println!("result:{}", result);

        // 因为closure引用了base的可变借用，所以borrow不能再取得base的可变借用了
        // let borrow = &mut base;
    }
    println!("******* move *******");
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

    println!("\n## 闭包引用可变借用，不改变，释放了闭包中的可变借用");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        {
            // 闭包closure所在的作用域
            let closure = |params| params + base;
            let result = closure(5_u32);

            // 因为没有改变base
            println!("base:{}", base);
            println!("result:{}", result);
        }

        println!("base:{}", base);

        // 虽然closure引用了base的可变借用
        // 但是随着闭包closure离开其作用域，也释放了闭包中base的可变借用
        // 所以borrow可以取得base的可变借用了
        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{}", borrow);
    }
    println!("******* move *******");
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

    println!("\n## 闭包引用可变借用，改变，释放了闭包中的可变借用");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        {
            // 闭包closure所在的作用域
            let mut closure = |params| {
                base += params;
                base
            };
            let result = closure(5_u32);

            // 因为改变了base
            // println!("base:{}", base);
            println!("result:{}", result);
        }

        println!("base:{}", base);

        // 虽然closure引用了base的可变借用
        // 但是随着闭包closure离开其作用域，也释放了闭包中base的可变借用
        // 所以borrow可以取得base的可变借用了
        let borrow = &mut base;
        *borrow = 20_u32;

        println!("borrow:{}", borrow);
    }
    println!("******* move *******");
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