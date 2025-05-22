fn main() {

    let ages: Vec<u8> = vec![21, 17, 54, 9, 87, 45, 66];

    for (i, age) in ages.iter().enumerate() {

        let real_index = i + 1;

        println!("Age_{real_index} -> {age}")
    }

}

// Age_1 -> 21
// Age_2 -> 17
// Age_3 -> 54
// Age_4 -> 9 
// Age_5 -> 87
// Age_6 -> 45
// Age_7 -> 66
