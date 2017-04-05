// 但是又会有另外一个问题：Box类型和FnOnce类型之间的配合不够默契
// Box<FnOnce()>这样的类型，调用时会发生编译错误
// RustTeam临时性设计了一个补丁std::boxed::FnBox trait
// 假如需要用 Box<FnOnce()>类型，请暂时改用 Box<FnBox()>类型

// （1）前往https://play.rust-lang.org/
// （2）选择Channel下的Nightly
// （3）运行下段代码

#![feature(fnbox)]

use std::boxed::FnBox;

fn test_fn_box() -> Box<FnBox(i32) -> i32> {
    let c = |num: i32| num * 2;
    Box::new(c)
}

fn test_fn() -> Box<Fn(i32) -> i32> {
    let c = |num: i32| num * 3;
    Box::new(c)
}

fn main() {
    {
        let closure_fn_box = test_fn_box();

        let result_1 = closure_fn_box(10);
        println!("result_1 = {}", result_1);

        let result_2 = closure_fn_box(100);
        println!("result_2 = {}", result_1);
    }

    {
        let closure_fn = test_fn();

        let result_1 = closure_fn(10);
        println!("result_1 = {}", result_1);

        let result_2 = closure_fn(100);
        println!("result_2 = {}", result_2);
    }
}