fn main() {

    let mut families_and_members: Vec<Vec<String>> = vec![vec!["Ayhan Bilir".to_string(), "Aysel Bilir".to_string()], vec!["Hakan Uzun".to_string(), "Kağan Uzun".to_string(), "Mehmet Uzun".to_string()]];

    // a baby here => jr Kutluk Bilir

    families_and_members[0].push("Kutluk Bilir".to_string());

    //let father_hakan = families_and_members[1][0]; // Error! => move occurs because value has type `String`, which does not implement the `Copy` trait

    let _father_hakan = &families_and_members[1][0];

    families_and_members[1][1] = "Kağan Mustafa Uzun".to_string();

    println!("Families and members: {:?}", families_and_members); // Families and members: [["Ayhan Bilir", "Aysel Bilir", "Kutluk Bilir"], ["Hakan Uzun", "Kağan Mustafa Uzun", "Mehmet Uzun"]]
}

