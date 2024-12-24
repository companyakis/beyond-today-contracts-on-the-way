fn main() {

    // heap data

    let city: String = "İzmir".to_string();

    println!("City: {city}"); // City: İzmir

    // built in drop function

    drop(city); // consider cloning the value if the performance cost is acceptable: `.clone()

    println!("City: {city}"); // error[E0382]: borrow of moved value: `city`  

}




