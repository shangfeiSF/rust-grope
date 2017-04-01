// Use the `into_iter()` function when you want to move, instead of borrow, your value.
// It is normally found of using `into_iter()` when there is a function that is transforming some values:

// The `into_iter() function` creates a IntoIter<T> type that now has ownership of the original value.
// Like Iter<'a, T>, it is this IntoIter<T> type that actually implements the Iterator trait.

// The word "into" is commonly used in Rust to signal that T is being moved.
// The docs also use the words owned or consumed interchangeably with moved.

fn transformers(persons: Vec<(String, usize)>) -> Vec<String> {
    persons.into_iter().map(|(name, _)| name).collect()
}

fn main() {
    {
        let persons = vec![
            ("David".to_string(), 50),
            ("Tony".to_string(), 40),
            ("Bob".to_string(), 30)
        ];
        let cloned_persons = persons.clone();
        let names = transformers(persons);

        assert_eq!(names, ["David", "Tony", "Bob"]);

        // persons has been moved at `transformers(persons);`
        // println!("persons = {:?}", persons);
        println!("cloned_persons = {:?}", cloned_persons);
        println!("names =  {:?}", names);
    }

    {}
}
