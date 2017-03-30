// 通过 derive 让 Rust 编译器自动实现 Copy/Clone特性
#[derive(Debug)]
#[derive(Copy, Clone)]

struct Test {
    number: f32,
    boolean: bool,
}

fn main() {
    println!("#通过 derive 让 Rust 编译器自动实现 Copy/Clone特性");

    let x = Test {
        number: 100.100_f32,
        boolean: true,
    };

    // Test 通过 derive 自动继承了 Copy特性和 Clone特性，所以
    // let mut y = x后，x并没有因为所有权move而出现不可访问错误
    let mut y = x;

    y.number = 200.200_f32;
    y.boolean = false;

    println!("x:{:?}", x);
    println!("y:{:?}", y);
}