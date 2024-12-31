fn main() {

    let years = [1990, 2018, 2020, 2022, 2024];

    let year1 = years.get(3);

    let years2 = years.get(100);

    println!("{:?}", year1);

    println!("{:?}",years2);
}

// Some(2022)
// None
