// Use the `iter()` function if you want to iterate over the values by reference.

// In this example, we are using `.map()` and `.fold()` to count the number of bytes for all strings in the `players` vector.
// We know that the `len()` function can use an immutable reference.
// As such, we prefer `iter()` instead of `iter_mut()` or `into_iter()`.
// This allows us to move the `players` vector to `log()` function later.

fn main() {
    println!("# 使用`.iter().map()`遍历迭代器中的每一元素的引用");

    println!("\n# 使用`for-in`和 `.iter()` 遍历迭代器中的每一个元素的引用");

    println!("\n## 没有索引");
    {
        let player_scores = [("Tony", 20), ("Mike", 23), ("Jane", 18), ("Dave", 19)];
        println!("Before player_scores = {:?}", player_scores);

        for player in player_scores.iter() {
            // *player = 100.100;
            print!("{:?}; ", player);
        }

        println!("\nAfter player_scores = {:?}", player_scores);
    }

    println!("\n## 配合`.enumerate()`提供索引");
    {
        let player_scores = [("Tony", 20), ("Mike", 23), ("Jane", 18), ("Dave", 19)];
        println!("Before player_scores = {:?}", player_scores);

        for (index, player) in player_scores.iter().enumerate() {
            // *player = 100.100;
            if index == 3 {
                print!("{}:{:?}", index, player);
            } else {
                print!("{}:{:?}; ", index, player);
            }
        }

        println!("\nAfter player_scores = {:?}", player_scores);
    }
}
