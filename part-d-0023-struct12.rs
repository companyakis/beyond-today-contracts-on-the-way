fn main() {

    // https://www.latlong.net/place/tokyo-japan-8040.html

    // 35.652832 - 139.839478
    
    let tokyo = City {
        name: "Tokyo".to_string(),
        country :"Japan".to_string(),
        location: Coordinates(35.652832, 139.839478)
    };

    println!("Tokyo info: {tokyo:?}");

    println!("{} - {}", tokyo.location.0, tokyo.location.1)

// Tokyo info: City { name: "Tokyo", country: "Japan", location: Coordinates(35.652832, 139.83948) }
// 35.652832 - 139.83948

}

#[derive(Debug)]
struct Coordinates(f32, f32);

#[derive(Debug)]
struct City {
    name: String,
    country: String,
    location: Coordinates
}


