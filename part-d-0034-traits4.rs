fn main() {
    let empy_ayhan = Employee {
        name: String::from("Ayhan Bilir"),
        department: String::from("HR"),
        title: String::from("Senior Expert"),
        salary_usd: 4_200,
    };

    empy_ayhan.print_info(); // Employee name: Ayhan Bilir - department: HR - title: Senior Expert - salary $4200

    let country_turkiye = Country {
        name: "Turkiye".to_string(),
        founder: "Mustafa Kemal Atatürk".to_string(),
        capital: "Ankara".to_string(),
    };

    country_turkiye.print_info(); // Country name: Turkiye, founder: Mustafa Kemal Atatürk and its capital: Ankara
}

trait Info {
    fn print_info(&self);
}

struct Employee {
    name: String,
    department: String,
    title: String,
    salary_usd: u16,
}

impl Info for Employee {
    fn print_info(&self) {
        println!(
            "Employee name: {} - department: {} - title: {} - salary ${}",
            self.name, self.department, self.title, self.salary_usd
        )
    }
}

struct Country {
    name: String,
    capital: String,
    founder: String,
}

impl Info for Country {
    fn print_info(&self) {
        println!(
            "Country name: {}, founder: {} and its capital: {}",
            self.name, self.founder, self.capital
        )
    }
}
