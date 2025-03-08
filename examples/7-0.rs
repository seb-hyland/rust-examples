fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    match b {
        0 => Err("Division by 0 attempted!"),
        _ => Ok(a / b)
    }
}

fn main() {
    let a = 3;
    let mut b = 0;

    let _result: Result<i32, &'static str> = safe_divide(a, b);

    // let _unwrapped_result: i32 = safe_divide(a, b).expect("b should never be 0");

    match safe_divide(a, b) {
        Ok(quotient) => println!("The quotient is: {quotient}"),
        Err(e) => eprintln!("{e}"),
    }

    b = 2;
    let my_vec = vec![3, 4, 6, 7, 10];
    println!("{:?}", multi_safe_divide_ugly(&my_vec, b));
    println!("{:?}", multi_safe_divide_better(&my_vec, b));
    println!("{:?}", multi_safe_divide_idiomatic(&my_vec, b));
}

fn multi_safe_divide_ugly(nums: &Vec<i32>, b: i32) -> Result<Vec<i32>, &'static str> {
    let mut result = Vec::new();
    
    for n in nums {
        match safe_divide(*n, b) {
            Ok(quotient) => result.push(quotient),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

fn multi_safe_divide_better(nums: &Vec<i32>, b: i32) -> Result<Vec<i32>, &'static str> {
    let mut result = Vec::new();

    for n in nums {
        let calculation_result: i32 = safe_divide(*n, b)?;
        result.push(calculation_result);
    }

    Ok(result)
}

fn multi_safe_divide_idiomatic(nums: &Vec<i32>, b: i32) -> Result<Vec<i32>, &'static str> {
    nums.iter().map(|n| safe_divide(*n, b)).collect()
}
