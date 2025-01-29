fn main() {

    // name, department, id, salary

    let her_data: (&str, &str, u8, u16) = ("Aygün Kaplan", "Sales", 101, 65000);

    println!("Her salary: {:?} ₺", her_data.3); // Her salary: 65000 ₺

    let (name, _, id, salary) = her_data; // _ => deparment

    println!("Name: {name} and id: {salary}"); // Name: Aygün Kaplan and id: 65000

}

