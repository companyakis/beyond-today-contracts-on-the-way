fn main() {

    let mut ages: Vec<u8> = vec![21, 17, 54, 9, 87, 45, 66];

    for (i, age) in ages.iter_mut().enumerate() {

        let real_index = i + 1;

        let older_age = *age + 5;

        println!("OlderAge_{real_index} -> {older_age}")
    }

}

// OlderAge_1 -> 26
// OlderAge_2 -> 22
// OlderAge_3 -> 59
// OlderAge_4 -> 14
// OlderAge_5 -> 92
// OlderAge_6 -> 50
// OlderAge_7 -> 71
