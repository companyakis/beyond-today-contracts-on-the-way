fn main() {
    // struct in struct

    let citizen_ayhan = Person {
        name: String::from("Ayhan Bilir"),
        country: String::from("Turkiye"),
        birty_year: 1975,
        work_info: ProLife {
            company: "Pro Sales Inc".to_string(),
            department: "Accounting".to_string(),
            title: "Senior Accounting Expert".to_string(),
            id: 4296,
        },

    };

    println!("Employee Ayhan ID: {}",citizen_ayhan.work_info.id ) // Employee Ayhan ID: 4296
}

struct ProLife {
    company: String,
    department: String,
    title: String,
    id: u16,
}

struct Person {
    name: String,
    country: String,
    birty_year: u16,
    work_info: ProLife,
}
