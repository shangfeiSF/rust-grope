// 手动实现 Copy/Clone特性
#[derive(Debug)]

struct Test {
    num_1: f32,
    bool_1: bool,
}

impl Copy for Test {}

impl Clone for Test {
    fn clone(&self) -> Test {
        Test {
            num_1: self.num_1,
            bool_1: self.bool_1,
        }
    }
}

fn main() {
    println!("#手动实现 Copy/Clone特性");

    let x = Test {
        num_1: 100.100_f32,
        bool_1: true,
    };

    // Test 手动实现了 Copy特性和 Clone特性，所以
    // let mut y = x后，x并没有因为所有权move而出现不可访问错误
    let mut y = x;

    y.num_1 = 200.200_f32;
    y.bool_1 = false;

    println!("x:{:?}", x);
    println!("y:{:?}", y);
}