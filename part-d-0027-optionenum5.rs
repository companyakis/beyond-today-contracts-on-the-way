fn main() {

    let _years: [u16; 5] = [1990, 2018, 2020, 2022, 2024];

    let _names = ["Mustafa".to_string(), "Aygün".to_string(), "Bilge".to_string(), "Kültigin".to_string(), "Bumin".to_string(), "Kutluk".to_string()];

    let year1 = _years.get(2);
    let year2 = _years.get(100);

    let name1 = _names.get(3);
    let name2 = _names.get(87);

    find_and_print_year(year1);
    find_and_print_year(year2);

    find_and_print_name(name1);
    find_and_print_name(name2);

}

// we can use a generic function
// look at => https://github.com/companyakis/ai-rust-with-python/blob/main/part-d-0026-generics2.rs

fn find_and_print_year(year: Option<&u16>) {

    match year {
        Option::Some(year) => println!("The year is: {year}"),
        Option::None => println!("No such a year...")
    }
}

fn find_and_print_name(name: Option<&String>) {

    match name {
        Option::Some(name) => println!("The name is: {name:?}"),
        Option::None => println!("No such a name!")
    }
}

// The year is: 2020
// No such a year...
// The name is: "Kültigin"
// No such a name!
