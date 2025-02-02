fn main() {

    let years: Vec<u16> = vec![2021, 2018, 2020, 2019, 2022, 2024, 2023];

    for year in years {

        println!("Year: {year}")
    }

    //println!("Years vector: {:?}", years) // Error => move occurs because `years` has type `Vec<u16>`, which does not implement the `Copy` trait

}

// Year: 2021
// Year: 2018
// Year: 2020
// Year: 2019
// Year: 2022
// Year: 2024
// Year: 2023
