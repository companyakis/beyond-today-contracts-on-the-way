fn main() {

    let mut fintech_dep: Area = Area { name: "FinTech Deparment".to_string(), head: "Mustafa Büyükdereli".to_string()};

    // Create a mutable reference

    let department: &mut Area = &mut fintech_dep;

    department.name = "FinTech and Innovation Department".to_string();

    println!("{:?}", fintech_dep) // Area { name: "FinTech and Innovation Department", head: "Mustafa Büyükdereli" }

} 

#[derive(Debug)]struct Area {
    name: String,
    head: String,
}


/*
 If you need to mutate data without copying and replacing it, then mutable references can help.

You can only have one mutable reference to a value simultaneously.

*/
