fn main() {

    let mut years: Vec<u16> = vec![2020, 2054, 2024];

    // push

    years.push(1990);
    years.push(2018);

    println!("{years:?}");

    // insert

    // insert at index 0

    years.insert(0, 1923);

    println!("{years:?}");

    // insert at index 2

    years.insert(2, 2000);

    println!("{years:?}");

}

// [2020, 2054, 2024, 1990, 2018]
// [1923, 2020, 2054, 2024, 1990, 2018]      
// [1923, 2020, 2000, 2054, 2024, 1990, 2018]


