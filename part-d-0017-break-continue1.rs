fn main() {

    let mut year: u16 = 1995;

    loop {

        if year == 1950 || year == 1942 {
            break;
        }

        println!("Year: {year}");

        year -= 5;
    }
}

// Year: 1995
// Year: 1990
// Year: 1985
// Year: 1980
// Year: 1975
// Year: 1970
// Year: 1965
// Year: 1960
// Year: 1955

