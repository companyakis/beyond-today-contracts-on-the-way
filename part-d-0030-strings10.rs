fn main() {
    
    let str_old = String::from("mustafa mustafa!");

    let str_new = str_old.replace("mustafa", "Mustafa");

    println!("The original string length is: {}, memory address: {:p}", str_old.len(), &str_old);

    println!("The new string length is: {}, memory address: {:p}", str_new.len(), &str_new);

}

// The original string length is: 16, memory address: 0xd6ed4f390
// The new string length is: 16, memory address: 0xd6ed4f3a8     
