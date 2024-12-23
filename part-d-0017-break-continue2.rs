fn main() {
    let mut year: u16 = 15;

    loop {
        
        if year < 5 {
            continue;
        }

        println!("Year: {year}");
        year -= 1;
    }
}

// Year: 15
// Year: 14
// Year: 13
// Year: 12
// Year: 11
// Year: 10
// Year: 9 
// Year: 8 
// Year: 7 
// Year: 6 
// Year: 5 
