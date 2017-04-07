use std::mem::{size_of, size_of_val, transmute};

fn main() {
    println!("## transmute实现类型转换\n");
    {
        println!("size_of::<&str>() = {:?}", size_of::<&str>());
        println!("size_of::<&[u8]>() = {:?}", size_of::<&[u8]>());

        let string = "abcdef";
        let u8_array = unsafe { transmute::<&str, &[u8]>(string) };

        println!("string = {:?}", string);
        println!("u8_array = {:?}", u8_array);
    }

    println!("\n## transmute实现类型转换");
    {
        let string = "abcdef";
        println!("\nsize_of_val(string) = {:?}", size_of_val(string));

        unsafe {
            let (ptr, len): (usize, isize) = transmute(string);
            println!("ptr: usize = {:?}", ptr);

            let ptr = ptr as *const u8; // ptr 转换为指针，从usize转换成*const u8，从十进制转换成十六进制
            println!("ptr as *const u8 = {:?}, len = {:?}", ptr, len);

            for i in 0..len {
                println!("ptr.offset({:?}) = {:?}, *ptr.offset({:?}) = {:?}", i, ptr.offset(i), i, *ptr.offset(i) as char);
            }
        }

        let string_slice = &string[2..4]; // ['c', 'd']
        println!("\nsize_of_val(string_slice) = {:?}", size_of_val(string_slice));

        unsafe {
            let (ptr, len): (usize, isize) = transmute(string_slice);
            println!("ptr: usize = {:?}", ptr);
            
            let ptr = ptr as *const u8;
            println!("ptr as *const u8 = {:?}, len = {:?}", ptr, len);

            for i in 0..len {
                println!("ptr.offset({:?}) = {:?}, *ptr.offset({:?}) = {:?}", i, ptr.offset(i), i, *ptr.offset(i) as char);
            }
        }

        let u8_array = unsafe { transmute::<&str, &[u8]>(string) };
        println!("\nsize_of_val(u8_array) = {:?}", size_of_val(u8_array));

        unsafe {
            let (ptr, len): (usize, isize) = transmute(u8_array);
            println!("ptr: usize = {:?}", ptr);
            
            let ptr = ptr as *const u8;
            println!("ptr as *const u8 = {:?}, len = {:?}", ptr, len);

            for i in 0..len {
                println!("ptr.offset({:?}) = {:?}, *ptr.offset({:?}) = {:?}", i, ptr.offset(i), i, *ptr.offset(i));
            }
        }

        let u8_array_slice = &u8_array[2..4]; // [99, 100]
        println!("\nsize_of_val(u8_array_slice) = {:?}", size_of_val(u8_array_slice));

        unsafe {
            let (ptr, len): (usize, isize) = transmute(u8_array_slice);
            println!("ptr: usize = {:?}", ptr);
            
            let ptr = ptr as *const u8;
            println!("ptr as *const u8 = {:?}, len = {:?}", ptr, len);

            for i in 0..len {
                println!("ptr.offset({:?}) = {:?}, *ptr.offset({:?}) = {:?}", i, ptr.offset(i), i, *ptr.offset(i));
            }
        }
    }
}