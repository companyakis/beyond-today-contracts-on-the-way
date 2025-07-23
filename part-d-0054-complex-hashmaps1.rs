use std::collections::HashMap;

    #[derive(Debug)]
    struct EmployeeDetails {

        name: String,
        department: String,
        salary_usd: u16,
        married: bool
    }

fn main() {

    // employeeID and employee details 

    let mut employees: HashMap<u16, EmployeeDetails> = HashMap::new();

    let mut employee_aydin_details: EmployeeDetails = EmployeeDetails { name: "Aydın Uçar".to_string(), department: "Sales".to_string(), salary_usd: 3_700, married: true};

    employees.insert(4296, employee_aydin_details);

    println!("Employees: {:?}", employees) // Employees: {4296: EmployeeDetails { name: "Aydın Uçar", department: "Sales", salary_usd: 3700, married: true }}
    
}

