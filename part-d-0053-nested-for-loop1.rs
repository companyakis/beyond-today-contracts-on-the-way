fn main() {

    let families_and_members: Vec<Vec<String>> = vec![vec!["Ayhan Bilir".to_string(), "Aysel Bilir".to_string()], vec!["Hakan Uzun".to_string(), "Kağan Uzun".to_string(), "Mehmet Uzun".to_string()]];

    for (family_index, family) in families_and_members.iter().enumerate() {

        println!("---------------Family {}------------------", family_index + 1);

        for (member_index, member) in family.iter().enumerate() {

            println!("Family Member: {} : {}", member_index + 1, member)
        }
    }
}

/*

---------------Family 1------------------
Family Member: 1 : Ayhan Bilir
Family Member: 2 : Aysel Bilir
---------------Family 2------------------
Family Member: 1 : Hakan Uzun
Family Member: 2 : Kağan Uzun
Family Member: 3 : Mehmet Uzun

*/
