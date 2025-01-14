fn main() {

    let name1 = String::from("Mustafa");
    let name2 = String::from("Aygün");
    let name3 = String::from("Hakan");

    let mut names = vec![name1, name2, name3];

    names[2] = "Kağan".to_string();

    println!("Names are: {names:?}"); // Names are: ["Mustafa", "Aygün", "Kağan"]

    let update_name = &mut names[1];

    update_name.push_str(" Büyükdereli");

    println!("Names are: {names:?}"); // Names are: ["Mustafa", "Aygün Büyükdereli", "Kağan"]

}






