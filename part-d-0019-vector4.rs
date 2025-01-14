fn main() {

    let name1 = String::from("Mustafa");
    let name2 = String::from("Aygün");
    let name3 = String::from("Hakan");

    let names = vec![name1, name2, name3];

    let search_name = names.get(22); // index number

    match search_name {
      
        Option::Some(n) => println!("The name is {n}."),
      
        None => println!("No name found. Try a different index number...")
    }

}






