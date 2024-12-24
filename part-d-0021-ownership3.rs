fn main() {

    // heap data

    let city: String = "İzmir".to_string();

    println!("City: {city}"); 

    // built in clone method
    // consider this way is not memory efficient

    let most_loved_city = city.clone();

    println!("Most loved city: {most_loved_city}"); 

    println!("City: {city}"); 

}

// City: İzmir
// Most loved city: İzmir
// City: İzmir

