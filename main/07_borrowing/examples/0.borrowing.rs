fn main() {
    println!("## 借用一次：let borrow = &base;");
    {
        let base = 1.1;

        let borrow = &base;

        println!("borrow:{:?}", borrow);
    }

    println!("\n## 借用两次：let borrow = &&base;");
    {
        let base = 2.1;

        let borrow = &&base;

        println!("borrow:{:?}", borrow);
    }

    println!("\n## 借用三次：let borrow = &&&base;");
    {
        let base = 3.1;

        let borrow = &&&base;

        println!("borrow:{:?}", borrow);
    }
}