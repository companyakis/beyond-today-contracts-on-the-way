fn main() {
    let employee_ayhan = Employee {
        name: "Ayhan Bilir".to_string(),
        department: "FinTech".to_string(),
        salary: 8500.0,
    };

    //Because of ownership, we can't them together
    // employee_ayhan.get_employee_info(); // Ayhan Bilir- FinTech- 8500
    // employee_ayhan.change_salary(); // Error => value used here after move

    //employee_ayhan.change_salary(); // Ayhan Bilir- FinTech- 10200

    employee_ayhan.change_salary_with_parameter(1.5); // Ayhan Bilir- FinTech- 12750
}

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
    salary: f32,
}

impl Employee {
    fn get_employee_info(self) {
        println!("{}- {}- {}", self.name, self.department, self.salary)
    }

    fn change_salary(mut self) {
        self.salary = self.salary * 1.2;

        println!("{}- {}- {}", self.name, self.department, self.salary) 
    }

    fn change_salary_with_parameter(mut self, rate: f32) {
        self.salary = self.salary * rate;

        println!("{}- {}- {}", self.name, self.department, self.salary) 
    }

}
