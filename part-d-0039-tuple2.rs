fn main() {

    // tuple in tuple

    // (first_name, last_name), id, salary

    let her_info: ((&str, &str), u8, u16) = (("Aygün", "Kaplan"), 101, 65000);

    let last_name = her_info.0.1;

    println!("Her last name is {last_name}..."); // Her last name is Kaplan...

}

