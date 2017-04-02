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

fn main() {
    {
        let closure_fn_box = test_fn_box();
        let result = closure_fn_box(2);
        println!("result = {}", result);
    }
}