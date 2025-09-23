fn main() {

    let info_fintect = Departments::FinTech(5).dep_info();

    println!("{}", info_fintect) // FinTech department head: Mustafa and the department has 5 employees.
}

#[derive(Debug)]
enum Departments {

    FinTech(u8),
    SalesMarketing(u8),
    Finance(u8)
}

impl Departments {

    fn dep_info(&self) -> String {

        match self {

            Departments::FinTech(a) => format!("FinTech department head: Mustafa and the department has {a} employees."),
            Departments::SalesMarketing(b) => format!("Sales and Marketing department head: Ayhan and the department has {b} employees."),
            Departments::Finance(c) => format!("Finance department head: Saygun and the department has {c} employees.")
        }
    }
}

    
