fn main() {

    let our_info = return_company_info();

    let _company_name = our_info.name;
    let _founder = our_info.founder;
    let _first_year = our_info.starting_year;

}

struct Company<'a> {
    name: &'a str,
    founder: &'a str,
    starting_year: u16,
}

fn return_company_info() -> Company<'static> {
    Company {
        name: "Company Akis",
        founder: "Mustafa Buyukdereli",
        starting_year: 2024,
    }
}
