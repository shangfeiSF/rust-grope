// Rust is immutable by default and iterators make it easy to manipulate data without needing mutability.
// If you do find yourself wanting to mutate some data
// You can use the iter_mut() function to get a mutable reference to the values.

fn main() {
    println!("# 使用`.iter_mut().map()`遍历迭代器中的每一元素的可变引用");
    {
        let mut teams = [[("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)],
                         [("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17)]];

        let teams_ordered_by_score = teams.iter_mut()
            .map(|team| {
                     team.sort_by(|&a, &b| a.1.cmp(&b.1).reverse());
                     team
                 })
            .collect::<Vec<_>>();

        // mutable borrow occurs at `teams.iter_mut()`
        // println!("teams = {:?}", teams);
        println!("teams_ordered_by_score = {:?}", teams_ordered_by_score);
    }

    println!("\n# 使用`for-in`和 `.iter_mut()` 遍历迭代器中的每一个元素的可变引用");

    println!("\n## 没有索引");
    {
        let mut teams = [[("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)],
                         [("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17)]];
        println!("Before teams = {:?}", teams);

        for team in teams.iter_mut() {
            *team = [team[1], team[2], team[3], team[0]];
            println!("{:?}; ", team);
        }

        println!("After teams = {:?}", teams);
    }

    println!("\n## 配合`.enumerate()`提供索引");
    {
        let mut teams = [[("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)],
                         [("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17)]];
        println!("Before teams = {:?}", teams);

        for (index, team) in teams.iter_mut().enumerate() {
            *team = [team[1], team[2], team[3], team[0]];

            if index == 1 {
                println!("{}:{:?}", index, team);
            } else {
                println!("{}:{:?}; ", index, team);
            }
        }

        println!("After teams = {:?}", teams);
    }

    println!("\n# 32个元素以上的array或者12个元素以上的元组");

    println!("\n## 使用`.iter_mut().map()`遍历迭代器");
    {
        let mut array: [&str; 33] = ["abc"; 33];
        println!("Before array[0..10] = {:?}", &array[0..10]);
        println!("Before array[30..33] = {:?}", &array[30..33]);

        let array_upper_case =
            array.iter_mut().map(|item| item.to_string().to_uppercase()).collect::<Vec<_>>();

        // mutable borrow occurs at `array.iter_mut()`
        // println!("After array[0..10] = {:?}", &array[0..10]);
        println!("After array_upper_case[0..10] = {:?}",
                 &array_upper_case[0..10]);
        println!("After array_upper_case[30..33] = {:?}",
                 &array_upper_case[30..33]);
    }

    println!("\n## 使用`for-in`和 `.iter_mut()` 遍历迭代器");
    {
        let mut array: [&str; 33] = ["abc"; 33];
        println!("Before array[0..10] = {:?}", &array[0..10]);
        println!("Before array[30..33] = {:?}", &array[30..33]);

        for item in array.iter_mut() {
            // *item = &item.to_string().to_uppercase()
        }

        println!("After array[0..10] = {:?}", &array[0..10]);
        println!("After array[30..33] = {:?}", &array[30..33]);
    }
}