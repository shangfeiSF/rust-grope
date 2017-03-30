// 手动实现 Copy/Clone特性
#[derive(Debug)]

struct Test {
    number: f32,
    boolean: bool,
}

impl Copy for Test {}

impl Clone for Test {
    fn clone(&self) -> Test {
        Test {
            number: self.number,
            boolean: self.boolean,
        }
    }
}

fn main() {
    println!("#手动实现 Copy/Clone特性");

    let x = Test {
        number: 100.100_f32,
        boolean: true,
    };

    // Test 手动实现了 Copy特性和 Clone特性，所以
    // let mut y = x后，x并没有因为所有权move而出现不可访问错误
    let mut y = x;

    y.number = 200.200_f32;
    y.boolean = false;

    println!("x:{:?}", x);
    println!("y:{:?}", y);
}