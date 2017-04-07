fn log(names: Vec<&str>, total: usize) {
    println!("names = {:?}", names);
    println!("total  = {:?}", total);
}

fn main() {
    let players = [("Tony", 20), ("Mike", 23), ("Jane", 18), ("Dave", 19)];

    println!("\n## 闭包的参数不需要声明借用类型");
    {
        // The closure used in `map()` does not require the `name` parameter to have a type
        // Notice that the type of `name` is `&&str` and not `&str`
        // Since the `iter()` function creates an iterator that has a reference to each element in the `names` vector.

        let names = vec!["Tony", "Mike", "Jane", "Dave"];

        // let total = names.iter().map(|name| name.len()).fold(0, |init, len| init + len);
        // let total = names.iter().map(|&name| name.len()).fold(0, |init, len| init + len);
        let total = names.iter().map(|name: &&str| name.len()).fold(0, |init, len| init + len);

        assert_eq!(names, ["Tony", "Mike", "Jane", "Dave"]);
        assert_eq!(total, 4 + 4 + 4 + 4);

        log(names, total);

        // value moved here: log(names, total);
        // println!("names = {:?}", names);
    }

    println!("\n## 闭包的参数需要声明借用类型");
    {
        // However, if we are destructuring the type, we do need to specify the reference

        // let names = players.iter().map(|(name, _age)| name).collect::<Vec<_>>();
        let names = players.iter().map(|&(name, _)| name).collect::<Vec<_>>();

        // let total = players.iter().map(|(_name, age)| age).fold(0, |init, age| init + age);
        let total = players.iter().map(|&(_, age)| age).fold(0, |init, age| init + age);

        assert_eq!(names, ["Tony", "Mike", "Jane", "Dave"]);
        assert_eq!(total, 20 + 23 + 18 + 19);

        log(names, total);

        // value moved here: log(names, total);
        // println!("names = {:?}", names);
    }
}