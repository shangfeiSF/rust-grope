use std::mem::{size_of, transmute};

fn main() {
    {
        let boxed = Box::new(());

        // unit 类型确实是 0 大小的类型
        println!("size_of::<()>() = {}", size_of::<()>());

        // 组成的数组，也是 0 大小类型
        println!("size_of::<[(); 100]>() = {}", size_of::<[(); 100]>());

        // 为 0 大小的类型申请动态分配内存，指针指向的地址是 1
        println!("{:p}", boxed);
        // 首先，1 不可能是内存分配器正常返回的地址
        // 其次，0 已经用于表示空指针 null 的情况
        // 所以选择另外一个不同的值来表示这种情况
    }

    {
        // 那么这两种“空”有什么区别呢?
        let x: Box<()> = Box::new(());
        let y: Option<Box<()>> = None;
        let z: Option<Box<()>> = Some(Box::new(()));

        unsafe {
            let value1: usize = transmute(x); // 非空指针指向 0 大小的类型，指向的是地址 1
            let value2: usize = transmute(y); // 空指针 null 都是指向的是地址 0
            let value3: usize = transmute(z); // 非空指针指向 0 大小的类型，指向的是地址 1

            println!("value1 = {}, value2 = {}, value3 = {}", value1, value2, value3);
        }
    }
}
