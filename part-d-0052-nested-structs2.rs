fn main() {
  
    // struct in struct and functions

    let citizen_bilge = create_citizen(
        "Bilhe Kağan".to_string(),
        "Turkiye".to_string(),
        1977,
        "Kek Bank".to_string(),
        "FinTech".to_string(),
        "Expert".to_string(),
        6254,
    );
}

fn create_citizen(
    name: String,
    country: String,
    birty_year: u16,
    company: String,
    department: String,
    title: String,
    id: u16,
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
        },
    }
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
