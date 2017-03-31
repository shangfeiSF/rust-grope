fn main() {
    println!("# 改变*borrow");

    println!("\n## 先可变借用一次，再借用一次：let borrow = & &mut base;");
    {
        let mut base = 1.1;
        let mut _another = 1.2;

        let borrow = &&mut base;
        // *borrow = &mut _another;

        println!("borrow:{:?}", borrow);
    }

    println!("\n## 先借用一次，再可变借用一次：let borrow = &mut &base;");
    {
        let mut base = 2.1;
        let _another = 2.2;

        let borrow = &mut &base;
        *borrow = &_another;

        println!("borrow:{:?}", borrow);
    }

    println!("\n## 交替使用可变借用和借用：let borrow = &mut & &mut &base;");
    {
        let mut base = 3.1;
        let mut _another = 3.2;

        // let temp = &mut &_another;
        let temp = &&mut &_another;
        let borrow = &mut &&mut &base;
        // *borrow = &temp;
        *borrow = temp;

        println!("{:?}", borrow);
    }

    println!("\n## 交替使用可变借用和借用：let borrow = &mut & &mut &base;");
    {
        let mut base = 4.1;
        let mut _another = 4.2;

        let borrow = &mut &&mut &base;
        // *borrow = &&mut &_another;

        println!("{:?}", borrow);
    }

    println!("\n## 交替使用可变借用和借用：let borrow = & &mut & &mut base;");
    {
        let mut base = 5.1;
        let mut _another = 5.2;

        let temp = &mut &&mut _another;
        let borrow = &&mut &&mut base;
        // *borrow = temp;

        println!("{:?}", borrow);
    }

    println!("\n## 交替使用可变借用和借用：let borrow = & &mut & &mut base;");
    {
        let mut base = 6.1;
        let mut _another = 6.2;

        let borrow = &&mut &&mut base;
        // *borrow = &mut &&mut _another;

        println!("{:?}", borrow);
    }
}