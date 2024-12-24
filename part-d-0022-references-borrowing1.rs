fn main() {

    // stack data

    let year = 1990; 

    let birth_year = year;

    let his_year = &year; // borrowing and referencing => NOT owner!

    println!("Year: {year}"); // Year: 1990

    println!("Birth year: {birth_year}"); // Birth year: 1990  

    println!("His year: {his_year}"); // His year: 1990

    // heap data

    let city: String = "İzmir".to_string(); // Owner

    println!("City: {city}"); // City: İzmir

    let most_loved_city = &city; // borrowing and referencing => NOT owner!

    println!("Most loved city: {most_loved_city}"); // Most loved city: İzmir

    println!("City: {city}"); // City: İzmir

}
