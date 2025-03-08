fn print_all_items<T>(items: T)
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    items.into_iter().for_each(|i| println!("{i}"));
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4];
    let words = ["hello", "world"];
    numbers.push(5);

    print_all_items(numbers);
    println!("----------");
    print_all_items(words);
}
