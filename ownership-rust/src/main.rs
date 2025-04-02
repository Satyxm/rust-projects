fn main() {
    // here, this is immutable - name
    let name: String = String::from("Alice");
    println!("Hello, {}!", name);

    //this is mutable
    let mut x_name: String = String::from("Bob");
    x_name.push_str(" The Builder");
    println!("Hello, {}!", x_name);

    //invalidated reference
    let s1: String = String::from("Hola");
    let s2 = s1;

   // println!("s1: {}", s1); // This will cause a compile-time error
    println!("s2: {}", s2); // This is valid, as s2 is the owner of the string now

    //Scope and definitions
    let mut soya: String = String::from("Hello");
    soya = String::from("World");
    println!("s: {}", soya); // This is valid, as s is the owner of the string now

    //usage of the clone
    let sumo: String = String::from("Sumo Wrestler");
    let mut sumo_clone:String = sumo.clone();
    println!("sumo: {}", sumo); // This is valid, as sumo is the owner of the string now
    println!("sumo_clone: {}", sumo_clone); // This is valid, as sumo_clone is a clone of the string now

    sumo_clone.push_str(" - The Great");
    println!("sumo_clone: {}", sumo_clone); // but, it works if i use 'mut' -- error: this will not work as sumo_clone is a clone of the string now

    //copy trait
    let x: i32 = 5;
    let y: i32 = x; // This is valid, as i32 implements the Copy trait
    println!("x: {}", x); // This is valid, as x is a primitive type and implements the Copy trait
    println!("y: {}", y); // This is valid, as y is a copy of x

    takes_ownership("this is a string".to_string());

    // println!("{}", sigma); - cause compile-time error

}

//ownership and functions
fn takes_ownership(sigma: String) {
    println!("val?: {}", sigma);

} // s goes out of scope and is dropped here