fn main() {
    let my_vec = vec![1, 3, 2, 4, 3, 5];

    let mut sum = 0;
    for n in my_vec.iter() {
        let result = n * 3;

        if n % 2 == 0 {
            sum += result;
        }
    }

    println!("The sum is {sum}");
}
