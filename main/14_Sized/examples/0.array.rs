// 数组(array)是一个容器，在一块连续内存空间中，存储了一系列的同样类型的数据
// 表示方式：[T; N]，T表示元素类型，N表示元素个数
// 数组(array)中元素占用空间大小必须是编译期确定的
// 数组(array)所容纳的元素个数也必须是编译期确定的

use std::mem::{size_of, transmute};

fn main() {
    {
        let origin: [i32; 5] = [1, 2, 3, 4, 5];

        fn modify(mut origin: [i32; 5]) {
            origin[0] = 100;
            println!("modified array = {:?}", origin);
        }

        // 数组 origin 传给函数 modify，origin 并不会退化成一个指针
        // 而是会将 origin 完整拷贝到 modify。函数体内对 origin 的改动，不会影响到外面
        modify(origin);

        println!("origin array = {:?}", origin);
    }

    {
        // 对数组取borrow操作，可以生成一个“数组切片(Slice)”
        // 数组切片(Slice)对数组没有“所有权”，可以看做是专门用于指向数组的指针，是对数组的另外一个“视图”
        let mut origin: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

        fn modify(slice: &mut [char]) {
            slice[0] = 'A';
            println!("modified array = {:?}", slice);
        }

        {
            // 数组[T; N]，它的借用指针的类型就是&[T; N]
            // 通过编译器内部魔法，转换为数组切片类型&[T]
            // 数组切片实质上还是指针，不过在类型系统中丢弃了编译阶段定长数组类型的长度信息
            // 而将此长度信息存储为运行期的值
            let slice: &mut [char; 5] = &mut origin;

            // &mut [char; 5]类型可以自动转换为&mut [char]数组切片类型传入modify
            modify(slice);
        }

        println!("origin array = {:?}", origin);
    }

    {
        // 自动转换是怎么实现的呢？
        // 因为&mut [char; 35]类型和&mut [char]类型在内部表示是有区别的
        // &mut [char; 5]是普通指针，数组长度信息是编译期确定的
        // &mut [char]是胖指针(fat pointer)，既可以指向[char; 3]数据，也可以指向[char; 4]数据，还能指向一个数组的一个切片

        // 胖指针(fat pointer)大小是普通指针的两倍
        // 存储了（1）所指向的地址（2）长度信息
        println!("size_of::<char>() = {:?}", size_of::<char>());
        println!("size_of::<&[char; 5]>() = {:?}", size_of::<&[char; 5]>());
        println!("size_of::<&[char]>() = {:?}", size_of::<&[char]>());

        let origin: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

        let slice: &[char] = &origin[2..4]; // ['c', 'd']

        println!("slice = {:?}", slice);

        unsafe {
            let (ptr, len): (usize, isize) = transmute(slice);
            let ptr = ptr as *const char;

            println!("ptr = {:?}, len = {:?}", ptr, len);

            for i in 0..len {
                // 一个char占4个字节：size_of::<char>()
                println!("ptr.offset({:?}) = {:?}, *ptr.offset({:?}) = {:?}", i, ptr.offset(i), i, *ptr.offset(i));
            }
        }
    }
}
