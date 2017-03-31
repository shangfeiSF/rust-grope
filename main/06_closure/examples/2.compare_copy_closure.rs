fn main() {
    println!("# 闭包引用了具有Copy特性的基本类型，如let base = 5_u32");

    println!("\n## 闭包引用<不可变借用>，再let borrow = &base;");
    println!("******* normal *******");
    {
        let base = 5_u32;

        // closure先copy base
        // 然后获取copyed base的不可变引用
        let closure = |params| params + base; // <不可变借用>
        let result = closure(5_u32);

        println!("result:{}", result);

        println!("base:{}", base);

        let borrow = &base;
        println!("borrow:{}", borrow);

        let borrow_another = &base;
        println!("borrow_another:{}", borrow_another);
    }
    println!("******* move *******");
    {
        let base = 5_u32;

        // closure先copy base
        // 然后通过move强制获取copyed base的所有权
        let closure = move |params| params + base; // <不可变借用>
        let result = closure(5_u32);

        println!("result:{}", result);

        println!("base:{}", base);

        let borrow = &base;
        println!("borrow:{}", borrow);

        let borrow_another = &base;
        println!("borrow_another:{}", borrow_another);
    }

    println!("\n## 闭包引用<不可变借用>，再let borrow = &mut base;");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let closure = |params| params + base; // <不可变借用>
        let result = closure(5_u32);

        println!("result:{}", result);

        println!("base:{}", base);

        // let borrow = &mut base;
        // *borrow = 20_u32;
        // println!("borrow:{}", borrow);
    }
    println!("******* move *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let closure = move |params| params + base; // <不可变借用>
        let result = closure(5_u32);

        println!("result:{}", result);

        println!("base:{}", base);

        let borrow = &mut base;
        *borrow = 20_u32;
        println!("borrow:{}", borrow);
    }

    println!("\n## 闭包引用<可变借用>");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let mut closure = |params| {
            // <可变借用>
            base += params;
            base
        };
        let result = closure(5_u32);

        println!("result:{}", result);

        // println!("base:{}", base);

        // let borrow = &mut base;
        // *borrow = 20_u32;
        // println!("borrow:{}", borrow);
    }
    println!("******* move *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        let mut closure = move |params| {
            // <可变借用>
            base += params;
            base
        };
        let result = closure(5_u32);

        println!("result:{}", result);

        println!("base:{}", base);

        let borrow = &mut base;
        *borrow = 20_u32;
        println!("borrow:{}", borrow);
    }

    println!("\n## 闭包引用<不可变借用>，再let borrow = & base;，<释放了闭包>");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        {
            let closure = |params| params + base;
            let result = closure(5_u32);

            println!("result:{}", result);

            println!("base:{}", base);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用
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

            println!("result:{}", result);

            println!("base:{}", base);
        }

        let borrow = &mut base;
        *borrow = 20_u32;
        println!("borrow:{}", borrow);
    }

    println!("\n## 闭包引用<可变借用>，再let borrow = &mut base;，<释放了闭包>");
    println!("******* normal *******");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        {
            let mut closure = |params| {
                base += params;
                base
            };
            let result = closure(5_u32);

            println!("result:{}", result);

            // println!("base:{}", base);
        }

        // 随着闭包closure离开其作用域，也释放了闭包中base的借用
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

            println!("result:{}", result);

            println!("base:{}", base);
        }

        let borrow = &mut base;
        *borrow = 20_u32;
        println!("borrow:{}", borrow);
    }
}