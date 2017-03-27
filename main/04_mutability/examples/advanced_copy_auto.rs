// 通过 derive 让 Rust 编译器自动实现 Copy/Clone特性
#[derive(Copy, Clone, Debug)]

struct Test {
    num_1: f32,
    bool_1: bool,
}

fn main() {
    println!("#通过 derive 让 Rust 编译器自动实现 Copy/Clone特性");

    let x = Test {
        num_1: 100.100_f32,
        bool_1: true,
    };

    let y = x;

    // Test 通过 derive 自动继承了 Copy特性和 Clone特性，所以
    // let y = x后，x并没有因为所有权move而出现不可访问错误
    println!("x:{:?}", x);
    println!("y:{:?}", y);
}