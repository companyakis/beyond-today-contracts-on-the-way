fn main() {

    let mut name = String::from("Mustafa");

    let name_mut_ref1 = &mut name;

    //let name_mut_ref2 = &mut name; // ^ second mutable borrow occurs here

    println!("My name is {}", name_mut_ref1);

    let name_mut_ref3 = &mut name;

    name_mut_ref3.push_str(" Büyükdereli");

    println!("My name is {}", name_mut_ref3); // My name is Mustafa Büyükdereli

    println!("My full name is {}", name) // My full name is Mustafa Büyükdereli 
}


