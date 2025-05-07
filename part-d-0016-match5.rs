fn main() {

    let my_department = dep_names_by_id(101);

    println!("My department is {my_department}.") // My department is FinTech Department.

}


fn dep_names_by_id(id: u8) -> String {

    match id {
        101 => "FinTech Department".to_string(),
        121 => "Sales Department".to_string(),
        81 => "Marketing Department".to_string(),
        96 => "HR Department".to_string(),
        112 => "Operations Department".to_string(),
        _ => "Unknown result".to_string()
    }
}
