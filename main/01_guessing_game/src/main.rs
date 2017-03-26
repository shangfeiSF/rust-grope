// Rust 标准库中未包含生成随机数的库
// Rust 团队提供了一个rand crate，一个"包装箱"（crate）是一个 Rust 代码的包
// rand 是一个"库包装箱"，一个"二进制包装箱"，也就是一个可执行文件，包含被其它程序使用的代码
// 使用外部包装箱是 Cargo 的亮点

// extern crate来让Rust知道正在使用 rand，这也等同于一个use rand
// extern 类似C++中用来表示变量或者函数定义在其他文件中
extern crate rand;

// 引入标准库io
use std::io;
use std::cmp::Ordering;
// 要使用一个方法Rng（如thread_rng），需要声明Rng在作用域中
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 在很多语言中，这叫做一个"变量声明"，不过 Rust 的称作"变量绑定"，其中暗藏玄机
        // 默认用let做变量绑定是不可变的，但是使用了mut会让一个绑定可变，而不是不可变的
        // let 并不是获取一个名字，事实上它接受一个模式（pattern）
        // ::是一个特定类型的"关联函数"，它与String自身关联，而不是与一个特定的String实例关联
        // String::new 也就是一些语言中的"静态方法"
        let mut guess = String::new();

        // io::stdin() 返回一个指向终端标准输入的句柄
        // 用这个句柄的read_line方法去获取用户输入
        // "方法"就像关联函数，不过只在一个类型的特定实例上可用，而不是这个类型本身
        // Rust有一个叫做"引用"的功能，它允许你对一片数据有多个引用，用它可以减少拷贝, &mut guess就是一个引用，引用默认是不可变的
        io::stdin()
            .read_line(&mut guess)
            // read_line方法返回一个值：io::Result
            // Rust的标准库中有很多叫做Result的类型：一个泛型Result，然后是子库的特殊版本，例如io::Result
            // io::Result有一个expect()方法获取调用它的值，而且如果它不是一个成功的值，panic!并带有你传递给它的信息
            // 如果去掉expect函数调用，程序会编译通过，不过会得到一个警告：io::stdin().read_line(&mut guess);
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {}是一个占位符，所以传递guess作为一个参数。如果有多个{}，应该传递多个参数
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}