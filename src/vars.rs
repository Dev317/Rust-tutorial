pub fn run() {
    let name = "Minh";
    let mut age = 21;
    println!("My name is {} and I am {}", name, age);

    age = 22;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Minh", 21);
    println!("{} is {}", my_name, my_age);
}