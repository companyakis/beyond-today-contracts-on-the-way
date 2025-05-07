fn main() {

    let izmir_location = set_location(38.423733, 27.142826);

    println!("İzmir latitude: {:?}", izmir_location.0);

    println!("İzmir longitude: {:?}", izmir_location.1);
}

#[derive(Debug)]
struct Location(f64, f64);

fn set_location(latitude: f64, longitude : f64) -> Location {

    Location(latitude, longitude)
}

// İzmir latitude: 38.423733
// İzmir longitude: 27.142826
