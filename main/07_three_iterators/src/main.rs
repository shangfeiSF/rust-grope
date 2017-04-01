// There are three common methods which can create iterators from a collection:
// 1. iter(), which iterates over &T.
// 2. iter_mut(), which iterates over &mut T.
// 3. into_iter(), which iterates over T.

fn main() {
    println!("## 使用`for item in &array`");
    {
        let array: [u32; 3] = [4, 5, 6];
        println!("Before array = {:?}", array);

        for item in &array {
            print!("{:?}; ", item);
        }

        println!("\nAfter array = {:?}", array);
    }

    println!("\n## 使用`for item in &mut array`");
    {
        let mut array = ['a', 'b', 'c'];
        println!("array = {:?}", array);

        for item in &mut array {
            *item = (*item as u8 + 5_u8) as char;
            print!("{:?}; ", item);
        }
        println!("array = {:?}", array);
    }

    println!("\n## 不是很好的方式（1）");
    {
        let mut seqences = ["1", "2", "3", "4"];
        println!("Before seqences = {:?}", seqences);

        let len = seqences.len();
        let mut _temp = [""; 4];

        for (index, n) in seqences.iter().enumerate() {
            if index + 1 < len {
                _temp[index + 1] = n.clone();
            } else {
                _temp[0] = n.clone()
            }
        }

        seqences = _temp;

        println!("_temp = {:?}", _temp);
        println!("After seqences = {:?}", seqences);
    }

    println!("\n## 不是很好的方式（2）");
    {
        let mut seqences = ["1", "2", "3", "4"];
        println!("Before seqences = {:?}", seqences);

        let len = seqences.len();
        let _origin = seqences;

        for (index, n) in seqences.iter_mut().enumerate() {
            if index == 0 {
                *n = _origin[len - 1];
            } else {
                *n = _origin[index - 1];
            }
        }

        println!("_origin = {:?}", _origin);
        println!("After seqences = {:?}", seqences);
    }
}