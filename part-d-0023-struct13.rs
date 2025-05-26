fn main() {

    // https://www.latlong.net/place/tokyo-japan-8040.html

    // 35.652832 - 139.839478
    
    let tokyo = City::new("Tokyo".to_string(), "Japan".to_string(), Coordinates(35.652832, 139.839478));

    tokyo.city_info(); // City: Tokyo and its coordinates: '35.652832 and 139.83948'

}

#[derive(Debug)]
struct Coordinates(f32, f32);

#[derive(Debug)]
struct City {
    name: String,
    country: String,
    location: Coordinates
}

impl City {
    
    fn new(name: String, country: String, location: Coordinates) -> Self {

        City { name, country, location }
    }

    fn city_info(&self) {

        println!("City: {} and its coordinates: '{} and {}'", self.name, self.location.0, self.location.1)
    }
}


