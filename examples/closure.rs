fn main() {
    let y = 5;

    let op1 = |x| x + y;
    let op2 = |x: i32| -> i32 { x / y };

    let result1 = helper(5, op1);
    let result2 = helper(5, op2);
    let result3 = helper(5, increment_by_one);

    println!("{result1}");
    println!("{result2}");
    println!("{result3}");
}

fn increment_by_one(value: i32) -> i32 {
    value + 1
}

fn helper(value: i32, operation: impl Fn(i32) -> i32) -> i32 {
    operation(value)
}
