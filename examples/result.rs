fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by 0 attempted!".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let a = 3;
    let b = 0;

    let _result: Result<i32, String> = safe_divide(a, b);

    // let _unwrapped_result: i32 = safe_divide(a, b).expect("b should never be 0");

    match safe_divide(a, b) {
        Ok(quotient) => println!("The quotient is: {quotient}"),
        Err(e) => eprintln!("{e}"),
    }
}
