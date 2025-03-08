fn main() {
    let my_array: [i32; 5] = [1, 3, 2, 4, 7];

    let _option: Option<&i32> = my_array.get(10);

    // let _unwrapped_option: &i32 = my_array.get(10).expect("my_array should definitely have 10 items");

    match my_array.get(10) {
        Some(n) => println!("The element is: {n}"),
        None => eprintln!("No element was found!"),
    }
}
