fn main() {

    let coffee: Drink = Drink {
        name: "Turkish Coffee".to_string(),
        price: 25.5,
        size: 's',
        hot: true
    };

    let favorite_drink = &coffee.name;

    println!("{}- {}- {}- {}", coffee.name, coffee.price, coffee.size, coffee.hot); // Turkish Coffee- 25.5- s- true

    println!("My favorite drink is {favorite_drink}."); // My favorite drink is Turkish Coffee.

    let coce = Drink {
        name: String::from("Pepsi"),
        price: 45.0,
        size: 'm',
        hot: false,
    };
}

struct Drink {
    name: String,
    price: f32,
    size: char, // 's', 'm', 'l
    hot: bool
}
