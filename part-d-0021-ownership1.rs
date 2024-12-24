fn main() {

    // stack data

    let year = 1990; 

    let birth_year = year;

    println!("Year: {year}"); // Year: 1990

    println!("Birth year: {birth_year}"); // Birth year: 1990  

    // heap data

    let city: String = "İzmir".to_string();

    println!("City: {city}"); // City: İzmir

    let most_loved_city = city;

    println!("Most loved city: {most_loved_city}"); // Most loved city: İzmir

    //println!("City: {city}"); // error[E0382]: borrow of moved value: `city`

}




