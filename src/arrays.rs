use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    print!("Single value: {}", numbers[0]);

    // Get arr length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice, copy the value from numbers
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("{:?}", numbers);
}