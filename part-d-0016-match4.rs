fn main() {

    let yeltsin: Presidents = Presidents::BorisYeltsin;

    let result = old_presidents_and_countries(yeltsin);

    println!("{result}") // Russia

}

enum Presidents {
  
    GeorgeBush,
    SaddamHussein,
    BorisYeltsin,
    TurgutOzal
}

fn old_presidents_and_countries(p: Presidents) -> String {

    match p {

        Presidents::GeorgeBush => "USA".to_string(),
        Presidents::SaddamHussein => "Iraq".to_string(),
        Presidents::BorisYeltsin => "Russia".to_string(),
        Presidents::TurgutOzal => "Turkiye".to_string(),
    }

}
