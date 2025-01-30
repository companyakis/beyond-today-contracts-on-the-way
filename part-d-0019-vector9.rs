fn main() {

    let nums: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

    //let search_item = &nums[100];

    //println!("Item: {search_item}") // thread 'main' panicked at src/main.rs:5:28:

    match nums.get(100) {
      
        Some(i) => println!("Item: {i}"),
        None => println!("The item does not exist!")
    }

}

