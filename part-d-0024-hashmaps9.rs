use std::collections::HashMap;

fn main() {

    let grades: Vec<(&str, u8)> = vec![
        ("Mustafa", 100),
        ("Aygün", 97),
        ("Bilge", 99)
    ];

    let mut grades_hashmap: HashMap<&str, u8>  = grades.into_iter().collect();

    // is Kağan there?

    // let kagan_grade: Option<&u8> = grades_hashmap.get("Kağan");

    // println!("Kağan grade: {:?}", kagan_grade); // Kağan grade: None

    // if we need numeric value?

    let kagan_grade2 = grades_hashmap.entry("Kağan").or_insert(0);

    println!("Kağan grade: {:?}", kagan_grade2); // Kağan grade: 0

    println!("Grades: {:?}", &grades_hashmap); // Grades: {"Bilge": 99, "Aygün": 97, "Kağan": 0, "Mustafa": 100}
}

