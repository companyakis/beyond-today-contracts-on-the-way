use std::collections::HashMap;

fn main() {

    let grades: Vec<(&str, u8)> = vec![
        ("Mustafa", 100),
        ("Aygün", 97),
        ("Bilge", 99)
    ];

    let mut grades_hashmap: HashMap<&str, u8>  = grades.into_iter().collect();

    println!("Grades hashmap: {:?}", grades_hashmap); // Grades hashmap: {"Aygün": 97, "Bilge": 99, "Mustafa": 100}

    println!("Aygün grade: {:?}", grades_hashmap["Aygün"]); // Aygün grade: 97

    // is Kağan there?

    let kagan_grade: Option<&u8> = grades_hashmap.get("Kağan");

    println!("Kağan grade: {:?}", kagan_grade); // Kağan grade: None
    
}

