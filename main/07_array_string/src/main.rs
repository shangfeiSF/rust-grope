// There are three common methods which can create iterators from a collection:
// 1. iter(), which iterates over &T.
// 2. iter_mut(), which iterates over &mut T.
// 3. into_iter(), which iterates over T.

fn main() {
    println!("## array.iter()");
    {
        let array: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("array:{:?}", array);

        for item in array.iter() {
            // *item = 100.100;
            println!("{}", item);
        }
    }

    println!("\n## array.iter().enumerate()");
    {
        let array: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("array:{:?}", array);

        for (index, item) in array.iter().enumerate() {
            // *item = 100.100;
            println!("{} : {}", index, item);
        }
    }

    println!("\n## array.iter_mut()");
    {
        let mut array: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("array:{:?}", array);

        for item in array.iter_mut() {
            *item = 100.100;
            println!("{}", item);
        }
    }

    println!("\n## array.iter_mut().enumerate()");
    {
        let mut array: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("array:{:?}", array);

        for (index, item) in array.iter_mut().enumerate() {
            *item = 100.100;
            println!("{} : {}", index, item);
        }
    }

    println!("\n## array.into_iter()");
    {
        let mut array: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("array:{:?}", array);

        let mut iter = array.into_iter();
        loop {
            let current = iter.next();
            match current {
                Some(n) => {
                    println!("{:?}, {}", current, n);
                }
                None => {
                    println!("END");
                    break;
                }
            };
        }
    }

    println!("\n## 修改可变array: [u32; 3]中每一项");
    {
        let mut array: [u32; 3] = [4, 5, 6];
        println!("array:{:?}", array);

        // for-in 遍历可写引用&mut array
        for item in &mut array {
            // 重写内存，而不是重新绑定
            *item += 10;
        }
        println!("array:{:?}", array);
    }

    println!("\n## 修改可变array: [char; 3]中每一项");
    {
        let mut array = ['a', 'b', 'c'];
        println!("array:{:?}", array);

        for item in &mut array {
            *item = (*item as u8 + 5_u8) as char;
        }
        println!("array:{:?}", array);
    }

    println!("\n## 修改可变array: [&str; 3]中每一项");
    {
        let mut array = ["Tom", "John", "Bob", "Lucy"];
        array[2] = array[0];
        array[0] = "David";
        println!("array:{:?}", array);

        let len = array.len();
        let mut array_tmp = ["", "", "", ""];

        for (index, item) in array.iter().enumerate() {
            if index + 1 < len {
                array_tmp[index + 1] = item.clone();
            } else {
                array_tmp[0] = item.clone()
            }
        }

        array = array_tmp;

        println!("array:{:?}", array);
        println!("array_tmp:{:?}", array_tmp);
    }

    println!("\n## 修改可变array: [&str; 3]中每一项");
    {
        let mut array = ["2", "1", "x", "4"];
        array[2] = array[0];
        array[0] = "0";
        println!("array:{:?}", array);

        let origin_array = array;

        for (index, item) in array.iter_mut().enumerate() {
            // let len = array.len();
            let len = 4;
            if index + 1 < len {
                *item = origin_array[index + 1];
            } else {
                *item = origin_array[0];
            }
        }

        println!("array:{:?}", array);
    }
}