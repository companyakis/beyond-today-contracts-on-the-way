fn main() {

    let name1 = String::from("Mustafa");
    let name2 = String::from("Aygün");
    let name3 = String::from("Hakan");

    // let names = vec![name1, name2, name3];

    // let mut new_names_vec = names;

    //println!("{:?}", names); // error[E0382]: borrow of moved value: `names`


    //let mut new_names_vec = names.clone();

    //println!("{:?}", names); // ["Mustafa", "Aygün", "Hakan"]


    //let mut new_names_vec = &names;

    //println!("{:?}", names); // ["Mustafa", "Aygün", "Hakan"]
    

    let mut names = vec![name1, name2, name3];

    let mut new_names_vec = &mut names;

    new_names_vec.push("Bilge".to_string());

    println!("{:?}", new_names_vec); // ["Mustafa", "Aygün", "Hakan", "Bilge"]

    println!("{:?}", names); // ["Mustafa", "Aygün", "Hakan", "Bilge"]

}






