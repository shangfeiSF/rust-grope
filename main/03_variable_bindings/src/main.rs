// 首先必须强调，Rust 没有变量的概念（Variable）

// Rust 通过 let 关键字声明标识符（Indetifier）
// Rust 通过 let 关键字将右侧（Right）的内存区域（Value）绑定（Binding）到左侧（Left）的标识符（Indetifier）

// let 关键字将标识符和内存区域”绑定”，绑定后这段内存区域就被这个标识符所拥有，这个标识符也成为这段内存的唯一所有者
// let a = 100具体过程如下：首先在栈内存上分配一个i32的资源，并填充值100
// 然后将这段内存区域与标识符a绑定，让标识符a成为这段内存区域的唯一所有者

fn main() {
    {
        // Rust 通过花括号定义作用域
        // Rust 规定在离开作用域时，局部标识符随即会被销毁
        // Rust 不同于C++和Java，会将局部标识符绑定的内存区域，连同其所有者（局部标识符）一起被销毁释放

        {
            let num: u32 = 5;
            println!("num:{:?}", num);
        }

        // 在离开 num 所在的作用域时，num和其绑定的内存区域一起被销毁
        // 在外层作用域中再次访问 num 就会提示无法找到 num 的错误，Rust 称这种代码为“释放后使用”
        // 这些错误都是在编译阶段发现的
        // println!("{:?}", num);
    }
    {
        // Rust 不会为标识符绑定默认的初始值
        // Rust 明确规定标识符绑定的初始值必须由程序员显式声明

        let num_uninitialized: u32;
        // 因为 num_uninitialized 没有显式声明绑定的初始值，会报错
        // println!("{:?}", num_uninitialized);

        num_uninitialized = 5;
        println!("num_uninitialized:{:?}", num_uninitialized);
    }
    {
        // let 绑定整数，默认类型推断是 i32
        let num_1 = 5;
        let num_2: i32 = 5;
        let num_3: u32 = 5;

        println!("num_1:{:?}, num_2:{:?}, num_3:{:?}", num_1, num_2, num_3);
        assert_eq!(num_1, num_2);
        // 因为类型不匹配，会报错
        // assert_eq!(num_1, num_3);
    }
    {
        // let 绑定浮点数，默认类型推断是f32
        let num_1 = 2.0;
        let num_2: f32 = 2.0;
        let num_3: f64 = 2.0;

        println!("num_1:{:?}, num_2:{:?}, num_3:{:?}", num_1, num_2, num_3);
        assert_eq!(num_1, num_2);
        // 因为类型不匹配，会报错
        // assert_eq!(num_1, num_3);
    }
    {
        // Rust 值类型的显式标记法，Rust 语法规定为 value+type 的形式
        let num_1 = 5u32;
        let num_2 = 2.0f64;

        let num_3: u32 = 5;
        let num_4: f64 = 2.0;

        assert_eq!(num_1, num_3);
        assert_eq!(num_2, num_4);

        println!("num_1:{:?}, num_2:{:?}", num_1, num_2);
        println!("num_1:{:?}, num_2:{:?}", num_3, num_4);
    }
    {
        // String 是一个带有的 vec:Vec<u8> 成员的结构体，你可以理解为 str 类型的动态形式
        let mut string_1 = String::new(); // String::new() 创建一个空的字符串

        println!("string_1:{:?}", string_1);

        string_1 = String::from("Hello "); // 将 `&str` 类型转化成 `String` 类型
        string_1.push('w'); // 压入字符（char）
        string_1.push_str("orld?"); // 压入字符串切片（&str）
        string_1.pop(); // 弹出字符（char）
        string_1.push('!'); // 压入字符（char）

        println!("string_1:{:?}", string_1);

        // Rust 中和“绑定”（Binding）相生相伴的另一个概念是“转移”（Move）
        // “转移”（Move）可以把所有权(ownership)从一个绑定转移到另一个绑定
        // “转移”（Move）同样通过 let 关键字实现，但是和“绑定”（Binding）不同，左侧和右侧均为标识符（Indetifier）
        let string_2 = string_1;

        println!("string_2:{:?}", string_2);
        // 被转移所有权的标识符不可以继续被使用，否则会造成被称为“转移后使用”的代码，而在编译阶段报错
        // println!("string_1:{:?}", string_1);
    }
}