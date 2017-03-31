fn main() {
    println!("## 不可变借用一次：let borrow = &base;");
    {
        let base = 1.1;

        let borrow = &base;

        println!("base:{:?}", base);
        println!("borrow:{:?}", borrow);
    }

    println!("\n## 不可变借用两次：let borrow = &&base;");
    {
        let base = 2.1;

        let borrow = &&base;

        println!("base:{:?}", base);
        println!("borrow:{:?}", borrow);
    }

    println!("\n## 不可变借用三次：let borrow = &&&base;");
    {
        let base = 3.1;

        let borrow = &&&base;

        println!("base:{:?}", base);
        println!("borrow:{:?}", borrow);
    }
}