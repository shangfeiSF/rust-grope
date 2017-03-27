// move关键字常用在闭包中，强制闭包获取所有权

fn main() {
    // （1）在闭包中，使用实现了Copy特性的基本类型
    {
        let base = 100_u32;

        // base绑定的是u32类型，属于实现了 Copy特性的基本类型
        // 在闭包中是否使用 move 没有影响

        // 在闭包 closure_1 使用 move 时，先 copy base 然后 move copyed-base
        //  println!引用 base 时未报错
        let closure_1 = move |params: u32| params + base;
        let added_1 = closure_1(100_u32);

        println!("base:{}, added_1:{}", base, added_1);

        // 在闭包 closure_2 不使用 move 时，先 copy base 然后使用的是copyed-base
        //  println!引用 base 时未报错
        let closure_2 = |params: u32| params + base;
        let added_2 = closure_2(200_u32);

        println!("base:{}, added_2:{}", base, added_2);
    }

    // （2）在闭包中，使用未实现Copy特性的String类型
    {
        let mut base_1: String = String::from("abc");

        // base_1绑定的是String类型，不属于实现了 Copy特性的基本类型
        // 在闭包中是否使用 move 是有影响的

        // 在闭包 closure_1 使用 move 时，将 base_1 的所有权 move 到闭包中
        //  println!引用 base_1 时会报错：value used here after move
        let mut closure_1 = move |params: char| base_1.push(params);
        closure_1('d');

        // println!("base_1:{:?}", base_1);

        let mut base_2: String = String::from("123");

        // 在闭包 closure_2 不使用 move 时，只是获取 base_2 的可变借用到闭包中，而不是获取 base_2 的所有权
        // 加了花括号作用域，是为了作用域结束后，base_2 的可变借用失效
        //  println!引用 base_ 2 时不会报错
        {
            let mut closure_2 = |params: char| base_2.push(params);
            closure_2('4');
        }

        println!("base_2:{:?}", base_2);
    }
}