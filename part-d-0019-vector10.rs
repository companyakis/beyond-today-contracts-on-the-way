fn main() {

    let mut nums: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

    // Iterate through and modify elements

    for n in &mut nums {

        *n *= 3;
    }

    println!("Updated vector: {nums:?}") // Updated vector: [3, 6, 9, 12, 15, 18]

}

