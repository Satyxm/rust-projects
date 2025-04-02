struct Ninja {
    name: String,
    age: u32,
}

fn main() {
    let ninja = Ninja {
        name: String::from("Naruto"),
        age: 17,
    };

    println!("Ninja's name: {}", ninja.name);
}

// i'll catch up later, gonna get some sleep :)