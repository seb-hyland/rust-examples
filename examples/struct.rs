#[derive(Debug, PartialEq)]
struct MyType {
    first_string: String,
    second_string: String,
    number: i32,
}

impl MyType {
    fn my_very_favourite_constructor() -> Self {
        MyType { first_string: "Hello".to_string(), second_string: ", world!".to_string(), number: 5 }
    }
}

impl std::fmt::Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        (0..self.number).for_each(|_| result += &format!("{}{}\n", self.first_string, self.second_string));
        write!(f, "{result}")
    }
}

fn main() {
    let obj1 = MyType::my_very_favourite_constructor();
    let obj2 = MyType { first_string: "Hello".to_string(), second_string: ", world!".to_string(), number: 5 };
    println!("Debug representation: {:?}", obj1);
    println!("Equality: {}", obj1 == obj2);
    println!("-------------");
    println!("{}", obj2);
}
