// What if we only wanted the first two names from that list?

fn main() {
    println!("## 全部clone()，再into_iter()，再take(2)");
    {
        let x = vec!["Jill", "Jack", "Jane", "John"];

        let y = x.clone()
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        println!("x = {:?}", x);
        println!("y = {:?}", y);
    }

    // If we clone all of x, then we are cloning all four elements, but we only need two of them.
    // We can do better by using `.map()` to clone the elements of the underlying iterator
    println!("\n## 全部iter()，再map(|i| i.clone())，再take(2)");
    {
        let x = vec!["Jill", "Jack", "Jane", "John"];

        // The Rust compiler can now optimize this code and only clone two out of the four elements of x.
        let y = x.iter()
            .map(|i| i.clone())
            .take(2)
            .collect::<Vec<_>>();

        println!("x = {:?}", x);
        println!("y = {:?}", y);
    }

    // This optimization is used so often that Rust core now has a special function that does this for us called `cloned()`
    println!("\n## 全部iter()，再cloned()，再take(2)");
    {
        let x = vec!["Jill", "Jack", "Jane", "John"];

        let y = x.iter()
            .cloned()
            .take(2)
            .collect::<Vec<_>>();

        println!("x = {:?}", x);
        println!("y = {:?}", y);
    }
}