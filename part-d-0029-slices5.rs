fn main() {

    let some_data = {

        let full_name= "Mustafa Büyükdereli";

        let years :[u16; 5] = [2025, 2020, 2018, 1997, 1993];

        format!("{} was here! Year is {}... His last name is {}.", &full_name[0..=6], &years[0], &full_name[7..full_name.len()])
    };

    println!("{}", some_data) // Mustafa was here! Year is 2025... His last name is  Büyükdereli.

}

