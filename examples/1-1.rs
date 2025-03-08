fn main() {
    let my_vec = vec![1, 3, 2, 4, 3, 5];

    let sum: i32 = my_vec.iter().map(|x| x * 3).filter(|x| x % 2 == 0).sum();

    println!("The sum is {sum}");
}
