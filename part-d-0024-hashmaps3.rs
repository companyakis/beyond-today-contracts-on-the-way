use std::collections::HashMap;

fn main() {

    let mut employees: HashMap<&str, u16> = HashMap::new(); // key and value pairs

    employees.insert("FinTech", 14);

    employees.insert("Sales", 4);

    employees.insert("Operations", 12);

    // remove

    let removed_dep = employees.remove("Sales");

    println!("{removed_dep:?}"); // Some(4)

    println!("{:?}", removed_dep.unwrap()); // 4

    println!("{:?}", employees); // {"Operations": 12, "FinTech": 14}

    let removed_dep = employees.remove("Sales");

    println!("{removed_dep:?}"); // None
    
}

