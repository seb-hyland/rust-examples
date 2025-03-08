trait Animal {
    fn sound(&self) -> &str;
    fn report_weight(&self) -> i32;
}

struct Dog {
    weight: i32,
}

impl Animal for Dog {
    fn sound(&self) -> &str {
        "Woof!"
    }

    fn report_weight(&self) -> i32 {
        self.weight
    }
}

struct Cat {
    weight: i32,
}

impl Animal for Cat {
    fn sound(&self) -> &str {
        "Meow..."
    }

    fn report_weight(&self) -> i32 {
        self.weight
    }
}

fn report_stats(animal: &dyn Animal) {
    println!("{} my weight is {}", animal.sound(), animal.report_weight());
}

fn main() {
    let my_dog = Dog { weight: 8 };
    let my_cat = Cat { weight: 4 };

    report_stats(&my_dog);
    report_stats(&my_cat);
}
