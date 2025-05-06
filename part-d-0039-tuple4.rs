fn main() {

    let mut emp_ayhan: (&str, &str, u16) = ("Ayhan Bilir", "Finance", 4_600);

    emp_ayhan.2 = 4_700;

    println!("Updated salary: ${}", emp_ayhan.2)


} 

// Updated salary: $4700
