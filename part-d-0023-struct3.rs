fn main() {

    let mut coffee: Drink = Drink {
        name: "Turkish Coffee".to_string(),
        price: 25.5,
        size: 's',
        hot: true
    };

    println!("{}- {}- {}- {}", coffee.name, coffee.price, coffee.size, coffee.hot); // Turkish Coffee- 25.5- s- true

    coffee.price = 27.5;

    println!("{}- {}- {}- {}", coffee.name, coffee.price, coffee.size, coffee.hot); //Turkish Coffee- 27.5- s- true

    let _coce = Drink {
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
