fn main() {

   let full_name = String::from("Mustafa Büyükdereli");

   let first_name = full_name.get(0..=6).unwrap();

   println!("{first_name}"); // Mustafa

   let updated_name = full_name.replace("Mustafa", "Mustafa Bilge");

   println!("{updated_name}") // Mustafa Bilge Büyükdereli

}

