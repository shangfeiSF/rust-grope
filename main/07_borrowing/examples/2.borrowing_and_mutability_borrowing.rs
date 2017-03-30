fn main() {
    println!("# 引用&borrow");

    println!("\n## 先可变借用一次，再借用一次：let borrow_r = & &mut base;");
    {
        let mut base = 1.1;

        let borrow = &&mut base;

        println!("{:?}", borrow);
    }

    println!("\n## 先借用一次，再可变借用一次：let borrow_r = &mut &base;");
    {
        let mut base = 2.1;

        let borrow = &mut &base;

        println!("{:?}", borrow);
    }

    println!("\n## 交替使用可变借用和借用：let borrow = &mut & &mut &base;");
    {
        let mut base = 3.1;

        let borrow = &mut &&mut &base;

        println!("{:?}", borrow);
    }

    println!("\n## 交替使用可变借用和借用：let borrow = & &mut & &mut base;");
    {
        let mut base = 4.1;

        let borrow = &&mut &&mut base;

        println!("{:?}", borrow);
    }
}