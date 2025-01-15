fn main() {

    let mut year: u16 = 2025;

    let mut year_up = || year += 1;

    year_up();
    year_up();
    year_up();

    println!("Updated year is {year}."); // Updated year is 2028.

    let mut year_down = || year -= 1;

    year_down();

    println!("Updated year is {year}."); // Updated year is 2027.

}

