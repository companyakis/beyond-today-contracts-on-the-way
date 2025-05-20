fn main() {

    let mut people_in_departments: Vec<Vec<String>> = Vec::new();

    people_in_departments.push(vec!["Ayhan Bilir".to_string(), "Hakan Güler".to_string()]);

    people_in_departments.push(vec!["Aygün Arslan".to_string(), "Kağan Yapar".to_string(), "Şelale Sultan".to_string()]);

    println!("All people: {:?}", people_in_departments)

}


//All people: [["Ayhan Bilir", "Hakan Güler"], ["Aygün Arslan", "Kağan Yapar", "Şelale Sultan"]]
