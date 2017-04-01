// Use the `iter()` function if you want to iterate over the values by reference.

// In this example, we are using `.map()` and `.fold()` to count the number of bytes for all strings in the `players` vector.
// We know that the `len()` function can use an immutable reference.
// As such, we prefer `iter()` instead of `iter_mut()` or `into_iter()`.
// This allows us to move the `players` vector to `log()` function later.

fn log(_players: Vec<&str>, _total: usize) {
    println!("log_players = {:?}", _players);
    println!("log_total  = {:?}", _total);
}

fn main() {
    println!("# 使用`.iter().map()`遍历迭代器中的每一元素的引用");

    println!("\n## 闭包的参数<不需要声明借用类型>");
    {
        let players = vec!["Tony", "Mike", "Jane", "Dave"];

        // The closure used in `map()` does not require the `player` parameter to have a type
        // Notice that the type of `player` is `&&str` and not `&str`
        // Since the `iter()` function creates an iterator that has a reference to each element in the `players` vector.

        // let total = players.iter().map(|player| player.len()).fold(0, |init, len| init + len);
        // let total = players.iter().map(|&player| player.len()).fold(0, |init, len| init + len);
        let total =
            players.iter().map(|player: &&str| player.len()).fold(0, |init, len| init + len);

        assert_eq!(players, ["Tony", "Mike", "Jane", "Dave"]);
        assert_eq!(total, 4 + 4 + 4 + 4);

        log(players, total);

        // value moved here: log(players, total);
        // println!("players = {:?}", players);
    }

    println!("\n## 闭包的参数<需要声明借用类型>");
    {
        let player_scores = [("Tony", 20), ("Mike", 23), ("Jane", 18), ("Dave", 19)];

        // However, if we are destructuring the type, we do need to specify the reference

        // let players = player_scores.iter().map(|(player, _age)| player).collect::<Vec<_>>();
        let players = player_scores.iter().map(|&(player, _)| player).collect::<Vec<_>>();

        // let total = player_scores.iter().map(|(_player, age)| age).fold(0, |init, age| init + age);
        let total = player_scores.iter().map(|&(_, age)| age).fold(0, |init, age| init + age);

        assert_eq!(players, ["Tony", "Mike", "Jane", "Dave"]);
        assert_eq!(total, 20 + 23 + 18 + 19);

        log(players, total);

        println!("player_scores = {:?}", player_scores);
    }

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
