fn main() {

    let my_dep: Departments = Departments::FinTech;

    println!("My department is {my_dep:?}...") // My department is FinTech...

}

#[derive(Debug)]
enum Departments {
    FinTech,
    Sales,
    Marketing,
    Operations,
    Accounting,
    Auditing,
    Finance
}

struct EmployeeInfo {
    id: u8,
    name: String,
    department: Departments
}


