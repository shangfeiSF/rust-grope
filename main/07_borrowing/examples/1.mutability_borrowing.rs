fn main() {
    println!("## 可变借用一次：let borrow = &mut base;");
    {
        let mut base = 1.1;

        let borrow = &mut base;
        *borrow = 10.1;

        println!("borrow:{:?}", borrow);
    }

    println!("\n## 可变借用两次：let borrow = &mut &mut base;");
    {
        let mut base = 2.1;

        let borrow = &mut &mut base;
        **borrow = 20.1;

        println!("borrow:{:?}", borrow);
    }

    println!("\n## 可变借用三次：let borrow = &mut &mut &mut base;");
    {
        let mut base = 3.1;

        let borrow = &mut &mut &mut base;
        ***borrow = 30.1;

        println!("borrow:{:?}", borrow);
    }
}