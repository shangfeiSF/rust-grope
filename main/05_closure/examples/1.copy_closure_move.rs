fn main() {
    println!("# <move>闭包引用了具有Copy特性的基本类型，如let base = 5_u32");

    println!("\n## <move>闭包引用<不可变借用>，let borrow = &base;");
    {
        let base = 5_u32;

        let closure = move |params| params + base; // <不可变借用>
        let result = closure(5_u32);

        println!("result:{}", result);

        println!("base:{}", base);

        let borrow = &base;
        println!("borrow:{}", borrow);

        let borrow_another = &base;
        println!("borrow_another:{}", borrow_another);
    }

    println!("\n## <move>闭包引用<不可变借用>，let borrow = &mut base;");
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

    println!("\n## <move>闭包引用<可变借用>");
    {
        let mut base = 0_u32;
        base = base + 5_u32;

        // A `mut` is required on `closure` because a `&mut` is stored inside.
        // Thus, calling the closure mutates the closure which requires a `mut`.
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

    println!("\n## <move>闭包引用<不可变借用>，let borrow = & base;，<释放了闭包>");
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

    println!("\n## <move>闭包引用<可变借用>，let borrow = &mut base;，<释放了闭包>");
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