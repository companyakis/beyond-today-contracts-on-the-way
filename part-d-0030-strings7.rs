fn main() {

    // split and into a vector

    let mut cities = " Ankara, İzmir, İstanbul, Adana, Kahramanmaraş     ";

    cities = cities.trim();

    let cities_vector: Vec<&str> = cities.split(", ").collect();

    println!("{:?}", cities_vector);

    for c in cities_vector.iter() {

        println!("City: {c}")
    }

}

// ["Ankara", "İzmir", "İstanbul", "Adana", "Kahramanmaraş"]
// City: Ankara       
// City: İzmir        
// City: İstanbul     
// City: Adana        
// City: Kahramanmaraş
