fn main() {
    
    let mut sentence: String = String::from("Man come here!");

    //sentence.insert(3, ',');

    //println!("{sentence}"); // Man, come here!

    sentence.insert_str(3, ", you should");

    println!("{sentence}"); // Man, you should come here!

}

