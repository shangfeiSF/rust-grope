// 栈内存（stack）从高位地址向下增长，且栈内存分配是连续的，一般对栈内存大小是有限制的
// 在Rust里，函数调用时会创建一个临时栈空间，调用结束后 Rust 会让这个栈空间里的对象自动进入 Drop 流程
// 栈顶指针自动移动到上一个调用栈顶，无需程序员手动干预，因而栈内存申请和释放是非常高效的

// 堆上内存（heap）是从低位地址向上增长，堆内存通常只受物理内存限制
// 而且通常是不连续的，一般手动申请和释放

// 系统将物理内存映射成虚拟地址空间，程序在启动时看到的虚拟地址空间是一块完整连续的内存
// 通常程序使用物理内存是不连续的

fn main() {
    // let error_result = error_concat("world");
    // println!("error_concated = {}", error_result);

    let correct_result = correct_concat("world");
    println!("correct_concat = {}", correct_result);
}

// 函数栈中创建的临时绑定，连同函数栈在函数调用结束后一起被销毁
// 如果返回一个临时绑定的引用，出现了“悬垂指针”现象，这是rust编译器不允许的

// fn error_concat(x: &str) -> &str {
//     let concated = "Hello ".to_string() + x;
//     &concated
// }

// String类型，使用堆内存（heap）存储，即便是在函数栈中创建的，但是
// 在函数返回时，函数栈销毁不影响String类型的存储，Rust利用浅拷贝，返回的实际上是“指针”

fn correct_concat(x: &str) -> String {
    let concated = "Hello ".to_string() + x;
    concated
}