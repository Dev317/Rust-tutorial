pub fn run() {
    // basic formatting
    println!("Number {}", 1);
    println!("{} is from {}", "Minh", "SG");

    // positional arguments
    println!("{0} is from {1} and likes to {2}", "Minh", "SG", "code");

    // named arguments
    println!("{name} likes to play {activity}" , name = "Minh", activity = "coding");

    // placeholder traits
    println!("Binary: {:b} Hex : {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}