enum Animal {
    Dog { weight: i32 },
    Cat { weight: i32 },
}

impl Animal {
    fn sound(&self) -> &str {
        match self {
            Animal::Dog { .. } => "Woof!",
            Animal::Cat { .. } => "Meow...",
        }
    }

    fn report_weight(&self) -> i32 {
        match self {
            Animal::Dog { weight } | Animal::Cat { weight } => *weight,
        }
    }
}

fn report_stats(animal: &Animal) {
    println!("{} my weight is {}", animal.sound(), animal.report_weight());
}

fn main() {
    let my_dog = Animal::Dog { weight: 8 };
    let my_cat = Animal::Cat { weight: 4 };

    report_stats(&my_dog);
    report_stats(&my_cat);
}
