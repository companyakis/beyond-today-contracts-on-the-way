use std::collections::HashMap;

fn main() {

    // from

    let emp_data: [(&str, u16); 3] = [("FinTech", 14), ("Sales", 4), ("Operations", 12)];

    let mut employees = HashMap::from(emp_data);

    println!("{employees:?}") // {"FinTech": 14, "Sales": 4, "Operations": 12}

    // let mut employees: HashMap<&str, u16> = HashMap::new(); // key and value pairs

    // employees.insert("FinTech", 14);

    // employees.insert("Sales", 4);

    // employees.insert("Operations", 12);

}

