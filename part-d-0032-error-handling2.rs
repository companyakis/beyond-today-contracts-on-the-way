fn main() {

    let years: Vec<u16> = vec![1997, 2007, 2018, 1990];

    let year_index= 22;

    match years.get(year_index) {

        Some(y) => println!("The year is {y}"),
        
        None => println!("We couldn't find the year you're looking for....")
        
    }

}
