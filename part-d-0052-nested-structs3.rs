fn main() {
  
    // struct in struct and functions
    // update example (a new function -> update_citizen_salary)

    // impl usage will be added! step by step...

    let mut citizen_bilge = create_citizen(
        "Bilge Kağan".to_string(),
        "Turkiye".to_string(),
        1977,
        "Kek Bank".to_string(),
        "FinTech".to_string(),
        "Expert".to_string(),
        6254,
        96_000
    
    );

    update_citizen_salary(&mut citizen_bilge, 105_000);

    println!("New salary: {}", citizen_bilge.work_info.salary) // New salary: 105000
}

fn update_citizen_salary(citizen: &mut Person, new_salary: u32) {
    citizen.work_info.salary = new_salary
}


fn create_citizen(
    name: String,
    country: String,
    birty_year: u16,
    company: String,
    department: String,
    title: String,
    id: u16,
    salary: u32
) -> Person {
    Person {
        name,
        country,
        birty_year,
        work_info: ProLife {
            company,
            department,
            title,
            id,
            salary
        },
    }
}

struct ProLife {
    company: String,
    department: String,
    title: String,
    id: u16,
    salary: u32
}

struct Person {
    name: String,
    country: String,
    birty_year: u16,
    work_info: ProLife,
}
