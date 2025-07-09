fn main() {

    let name = String::from("Mustafa");

    let ref_name1 = &name;
    let ref_name2 = &name;
    let ref_name3 = &name;
    let ref_name4 = &name;
    let ref_name5 = &name;

    println!("{ref_name1} {ref_name2} {ref_name3} {ref_name4} {ref_name5}"); // Mustafa Mustafa Mustafa Mustafa Mustafa

    let my_name = name;

    // println!("My name is {name}"); // error[E0382]: borrow of moved value: `name`
}
