fn main() {

    let employee_ayhan = employee_data("Ayhan Bilir".to_string(), "HR".to_string(), 4_300);

    println!("Salary of emp Ayhan: {}", employee_ayhan.2) // Salary of emp Ayhan: 4300
    
}


fn employee_data(name: String, department: String, salary_usd: u16) -> (String, String, u16) {

    (name, department, salary_usd)
}
