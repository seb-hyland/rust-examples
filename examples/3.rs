#[derive(Debug, PartialEq)]
struct MyType {
    first_string: &'static str,
    second_string: &'static str,
}

impl MyType {
    fn my_very_favourite_constructor() -> Self {
        MyType {
            first_string: "Hello",
            second_string: ", world!",
        }
    }

    fn print_first_string(&self) {
        println!("{}", self.first_string)
    }

    fn change_first_string(&mut self, new_string: &'static str) {
        self.first_string = new_string
    }
}

impl std::fmt::Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.first_string, self.second_string)
    }
}

fn main() {
    let obj1 = MyType::my_very_favourite_constructor();
    let mut obj2 = MyType {
        first_string: "Happy",
        second_string: " birthday!",
    };

    println!("-------------");
    println!("Debug representation: {:?}", obj1);
    println!("Equality: {}", obj1 == obj2);
    println!("-------------");

    println!("My own display implementation: {obj2}");
    obj2.print_first_string();
    println!("-------------");

    obj2.change_first_string("Sad");
    println!("Changed object 2: {obj2}");
}
