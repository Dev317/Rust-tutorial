use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    print!("Single value: {}", numbers[0]);

    // Get arr length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice, copy the value from numbers
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("{:?}", numbers);

    // Loop through vector values
    for elem in numbers.iter() {
        println!("Number: {}", elem);
    }

    // Loop and mutate values
    for elem in numbers.iter_mut() {
        *elem *= 2;
    }

    println!("Number vec: {:?}", numbers);
}