fn main() {

    let fintech_dep = dep_info(Departments::FinTech, "Mustafa Büyükdereli".to_string(), 12);

}

enum Departments {
    FinTech,
    Sales,
    Marketing,
    Finance,
    HR,
    Audit,
    Operations
}

struct DepInfo {
    name: Departments,
    head: String,
    number_of_employees: u8
}

fn dep_info(name: Departments, head: String, emp_number: u8) -> DepInfo {

    DepInfo { name: name, head: head, number_of_employees: emp_number}
}
