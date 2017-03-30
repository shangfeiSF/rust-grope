// （1）let mut 声明绑定是否可变（可变：标识符绑定到新的内存区域）
// （2）&mut 声明引用是否可变（可变：通过*引用改变内存区域的资源）

fn main() {
    println!("## 在一个作用域中，&T和&mut T不能同时存在");
    {
        let mut base = 'a';

        let _read = &base;
        // let _write = &mut base;
    }

    println!("\n## 在一个作用域中，可以同时存在多个&T");
    {
        let base = 'a';

        let _read_1 = &base;
        let _read_2 = &base;
        let _read_3 = &base;
    }

    println!("\n## 在一个作用域中，只能存在一个&mut T");
    {
        // let base = 'a';
        let mut base = 'a';

        let _write_1 = &mut base;
        // let _write_2 = &mut base;
        // let _write_3 = &mut base;
    }

    println!("\n## let borrow = &base; 不能改变*borrow");
    {
        let base = 1.1;

        let borrow = &base;
        // *borrow = 10.1;

        println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let borrow = &base; 不能改变borrow");
    {
        let base = 1.2;
        let another = 10.2;

        let borrow = &base;
        // borrow = &another;

        println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let borrow = &mut base; 可以改变*borrow");
    {
        let mut base = 2.1;

        let borrow = &mut base;
        *borrow = 20.1;

        // println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let borrow = &mut base; 不能改变borrow");
    {
        let mut base = 2.2;
        let mut another = 20.2;

        let borrow = &mut base;
        // borrow = &mut another;

        // println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let mut borrow = &base; 不能改变*borrow");
    {
        let base = "a1";

        let mut borrow = &base;
        // *borrow = "A1";

        println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let mut borrow = &base; 可以改变borrow");
    {
        let base = "a2";
        let another = "A2";

        let mut borrow = &base;
        borrow = &another;

        println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let mut borrow = &mut base; 可以改变*borrow");
    {
        let mut base = "b1";

        let mut borrow = &mut base;
        *borrow = "B1";

        // println!("base:{}", base);
        println!("borrow:{}", borrow);
    }

    println!("\n## let mut borrow = &mut base; 可以改变borrow");
    {
        let mut base = "b2";
        let mut another = "B2"; // 必须是let mut

        let mut borrow = &mut base;
        borrow = &mut another; // 必须是&mut

        // println!("base:{}", base);
        println!("borrow:{}", borrow);
    }
}