fn main() {

    let _years = [1990, 2018, 2020, 2022, 2024];

    let _names = ["Mustafa", "Aygün", "Bilge", "Kültigin", "Bumin", "Kutluk"];

    let year1 = _years.get(2);
    let year2 = _years.get(100);

    let name1 = _names.get(3);
    let name2 = _names.get(87);


    find_and_print_item(year1);
    find_and_print_item(year2);

    find_and_print_item(name1);
    find_and_print_item(name2);

}

fn find_and_print_item<T :std::fmt::Debug>(item: Option<T>) {

    match item {
        Option::Some(i) => println!("The item is: {:?}", i),
        Option::None => println!("No result...")
    }
}

// The item is: 2020
// No result...
// The item is: "Kültigin"
// No result...
