use std::thread;

fn main() {
    {
        // spawn函数接受的参数是一个闭包，闭包里面引用了函数体内的局部变量
        // 闭包是运行在另外一个线程上，编译器无法肯定局部data的生命周期一定大于闭包的生命周期
        // 于是发生了错误

        // let mut data = 10;
        // thread::spawn(|| {
        //     data *= 2;
        // });
        // println!("{}", data);
    }

    {
        // 用move修饰闭包，关于生命周期的编译错误会消失
        // 但是执后data并未发生改变
        // 因为data是实现了Copy trait，在move闭包时候，闭包内一份拷贝，没有被真正修改

        let mut data = 10;
        let join_handle = thread::spawn(move || {
            data *= 2;
            println!("change data in tread: data = {}", data);
        });
        join_handle.join().unwrap();
        println!("data out thread: data = {}", data);
    }

    {
        let mut data: Vec<u32> = vec![1, 2];
        let join_handle = thread::spawn(move || {
            data.push(3);
            println!("change data in tread: data = {:?}", data);
        });
        join_handle.join().unwrap();

        // 既然已经把data move到闭包
        // 那么就不能再继续使用了

        // 不可读
        // println!("data out thread: data = {:?}", data);
        // 不可写
    }

    // 综合上述三个例子，没有办法在多线程中直接读写共享变量，除非使用Rust提供的线程安全相关的方法
}