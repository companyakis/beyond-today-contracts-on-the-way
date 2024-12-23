fn main() {

    let mut years: Vec<u16> = vec![1923, 2020, 2000, 2054, 2024, 1990, 2018];

    // pop

    years.pop();

    println!("{years:?}");

    years.pop();

    println!("{years:?}");

    // remove

    years.remove(1); // index num

    println!("{years:?}");

}

// [1923, 2020, 2000, 2054, 2024, 1990]
// [1923, 2020, 2000, 2054, 2024]
// [1923, 2000, 2054, 2024]   




