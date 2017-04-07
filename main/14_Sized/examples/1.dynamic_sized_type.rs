// 对于编译阶段大小不定的类型，Rust将其称之为 Dynamic Sized Type
// 不能直接声明 DST 类型的变量绑定，因为编译器根本没办法知道，怎么为它分配内存
// 但是，指向这种类型的指针是可以存在的，因为指针的大小是固定的

// Rust中有一个重要的 trait Sized，可以用于区分一个类型是不是 DST
// 所有的 DST 类型都不满足 Sized 约束

// 在泛型约束中使用 Sized、!Sized、?Sized
// （1）T:Sized 代表类型必须是编译期确定大小的
// （2）T:!Sized 代表类型必须是编译期不确定大小的
// （3）T:?Sized 代表以上两种情况都可以
// 在泛型代码中，泛型类型参数默认携带了 Sized 约束，因为这是最普遍最常见的情况
// 如果希望泛型参数可以支持 DST 类型，那么就应该加上 ?Sized 约束

use std::fmt::Debug;
use std::mem::{size_of, transmute};

#[allow(dead_code)]
fn test_sized<T>(chars: &T)
    where T: Debug
{
    println!("{:?}", chars);
}

fn test_both_sized<T: ?Sized>(chars: &T)
    where T: Debug
{
    println!("{:?}", chars);
}

// 直接在语言中加入对 DST 的支持是有好处的
// 虽然这种类型无法直接实例化，但是可以被用在 impl 块，以及泛型代码中
// 比如，可以为 [i32] 类型 impl 一个 trait
// 比如， Rc<[i32]> 也是一个合法的类型，为 [i32] 类型添加的方法，自然而然就可以被 Rc<[i32]> 使用



// 还有一种常见的 DST 类型就是 trait。
// rait 仅仅规定了类型需要实现的方法，而对具体类型的大小没有限制
// 因此实现同一个 trait 的具体类型大小是不定的，所以我们不能直接声明 trait 类型的变量
// 把 trait 放到指针后面是合法的，此时，指针也是胖指针
// 其中包含了指向真实数据结构的指针以及指向虚函数表的 vtable 指针
// 这种胖指针，也叫做 trait object

fn main() {
    {
        let chars: &[char] = &['a', 'b', 'c', 'd'];
        // test_sized(chars);
        test_both_sized(chars);
    }

    {
        // Rust 中的 str 类型也是一种典型的 DST 类型
        // 跟不定长数组是一样的，它内部就是一个 u8 类型的不定长数组
        // &str也是一个胖指针，跟数组切片一模一样

        let string = "Hello DST";
        println!("size_of::<u8>() = {:?}", size_of::<u8>());

        unsafe {
            let (ptr, len): (usize, isize) = transmute(string);
            let ptr = ptr as *const u8;

            println!("ptr = {:?}, len = {:?}", ptr, len);

            for i in 0..len {
                // 一个u8占1个字节：size_of::<u8>()
                println!("ptr.offset({:?}) = {:?}, *ptr.offset({:?}) = {:?}", i, ptr.offset(i), i, *ptr.offset(i) as char);
            }
        }
    }
}