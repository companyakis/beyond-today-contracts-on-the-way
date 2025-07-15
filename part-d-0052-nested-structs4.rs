fn main() {

    let mut citizen_kagan = Person {
      
        name: "Kagan Bilir",
        country : "Turkiye",
        birth_year: 1975,
        work_info: ProLife {
            company: "ABC Company",
            department: "Sales",
            title: "Sales Expert",
            id: 1245,
            salary: 72_000
        }
    };

    citizen_kagan.update_citizen_salary(87200);

    println!("{:?}", citizen_kagan); // Person { name: "Kagan Bilir", country: "Turkiye", birth_year: 1975, work_info: ProLife { company: "ABC Company", department: "Sales", title: "Sales Expert", id: 1245, salary: 87200 } }
    
}

#[derive(Debug)]
struct ProLife {
    company: &'static str,
    department:  &'static str,
    title:  &'static str,
    id: u16,
    salary: u32
}

#[derive(Debug)]
struct Person {
    name:  &'static str,
    country:  &'static str,
    birth_year: u16,
    work_info: ProLife,
}

impl Person {

    fn update_citizen_salary(&mut self, new_salary: u32) {
      
        self.work_info.salary = new_salary;
    }
}
