fn main() {

    let name = String::from("Mustafa");

    let name_ref1 = &name;

    let name_ref2 = &name_ref1;

    let name_ref3 = &name_ref2;

    println!("My name is {name}");

    println!("My name is {name_ref1}"); // Deref coercion

    println!("My name is {}", *name_ref1);

    println!("My name is {}", **name_ref2);

    println!("My name is {}", ***name_ref3);

    println!("Stack address {:p}", name_ref1);
}
