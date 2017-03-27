fn main() {
    // （1）不可变绑定和可变绑定
    {
        println!("#不可变绑定和可变绑定");

        let num_1 = 5_u32;
        let mut num_2 = 10_f32;

        println!("num_1:{}", num_1);
        println!("num_2:{}", num_2);

        // Rust 规定不能对不可变绑定赋值，如果要修改，必须用关键字 mut 声明绑定为可变的
        // num_1 = 100_u32;
        num_2 = 2000_f32;

        println!("num_1:{}", num_1);
        println!("num_2:{}", num_2);
    }

    // （2）不可变绑定和const的区别
    {
        print!("#不可变绑定和const的区别");
        // ”不可变绑定“用来约束绑定行为，不能通过“所有者”（标识符）更改对应的内存区域
        let array = vec![1, 2, 3];

        // array 通过 mut 称为可变绑定，绑定的内存区域没有改变，可以通过新的 array 修改内存区域了
        let mut array = array;

        array.push(4);
        println!("array:{:?}", array);

        let mut string: &str = "abc";
        println!("string:{:?}", string);

        // string 本身就是可变绑定，绑定到新的内存区域，即修改了 string
        string = "xyz";

        println!("string:{:?}", string);

        // Rust 也有 const 常量，常量不存在“绑定”，和其他语言的常量含义相同

        const PI: f32 = 3.1415926;

        println!("PI:{:?}", PI);
    }

    // （3）高级Copy
    {
        println!("#高级Copy");

        // Copy特性定义在标准库 std::marker::Copy 中
        // 一种类型实现了Copy特性，会先拷贝内容到新内存区域，然后把新内存区域和标识符做绑定

        // 哪些自定义的类型可以实现Copy特性？
        // 只要这种类型的属性类型都实现了Copy特性，那么这个类型就可以实现Copy特性

        // _Foo 可实现Copy特性
        // i32 和 bool 均实现了Copy特性，所以 _Foo 可以实现Copy特性
        struct _Foo {
            num_1: f32,
            bool_1: bool,
        }

        // _Bar 不可实现Copy特性
        // Vec<T>并没有实现Copy特性，所以 _Bar 无法实现Copy特性
        struct _Bar {
            array: Vec<i32>,
        }

        println!("## 执行自动实现Copy/Clone特性：cargo run --example advanced_copy_auto");
        println!("## 执行手动实现Copy/Clone特性：cargo run --example advanced_copy");
    }
}