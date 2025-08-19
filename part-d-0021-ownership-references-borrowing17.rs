fn main() {

    // Only one mutable reference can exist to that data in a particular scope at any given time.
    
    let mut sentence: String = "A barking dog never bites.".to_string();
    
    println!("{sentence}");
    
    let ref_sentence1  = &mut sentence;
    
    //let ref_sentence2 = &mut sentence;
    
    println!("{ref_sentence1}");
    
    *ref_sentence1 = String::from("Out of sight, out of mind.");
    
    println!("{ref_sentence1}");
    
    println!("{sentence}");
    
    //println!("{ref_sentence2}");
    
}

// A barking dog never bites.
// A barking dog never bites.
// Out of sight, out of mind.
// Out of sight, out of mind.
