// A for loop in Rust is really just syntatic sugar around `.into_iter()` function.

fn main() {
    println!("## into_iter()和loop实现遍历Vec<T>");
    {
        let values = vec![1, 2, 3, 4];
        println!("Before values = {:?}", values);

        let mut iter = values.into_iter();
        loop {
            match iter.next() {
                Some(x) => print!("{}; ", x),
                None => break,
            }
        }

        // value moved here a: values.into_iter();
        // println!("\nAfter values = {:?}", values);
    }

    println!("\n\n## into_iter()和loop实现遍历[T; N]");
    {
        let array: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("Before array = {:?}", array);

        let mut iter = array.into_iter();
        loop {
            match iter.next() {
                Some(n) => print!("{}; ", n),
                None => break,
            };
        }

        // copyed-array moved here: array.into_iter();
        println!("\nAfter array = {:?}", array);
    }

    println!("\n## for-in实现遍历，moved");
    {
        let values = vec![1, 2, 3, 4];
        println!("Before values = {:?}", values);

        for x in values {
            print!("{}; ", x);
        }

        // values has been moved here: `for x in values`
        // println!("\nAfter values = {:?}", values);
    }

    println!("\n\n## for-in实现遍历，none moved");
    {
        let values = vec![1, 2, 3, 4];
        println!("Before values =  {:?}", values);

        // If we want to use values after the for loop, we just need to use a reference instead
        for x in &values {
            print!("{}; ", x);
        }

        println!("\nAfter values = {:?}", values);
    }
}