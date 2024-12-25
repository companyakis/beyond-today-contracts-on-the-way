use std::collections::HashMap;

fn main() {

    let mut employees: HashMap<&str, u16> = HashMap::new(); // key and value pairs

    employees.insert("FinTech", 14);

    employees.insert("Sales", 4);

    employees.insert("Operations", 12);

    println!("Number of employees: {employees:?}") // Number of employees: {"FinTech": 14, "Operations": 12, "Sales": 4}

}
