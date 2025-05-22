use rand::Rng;

fn main() {

    // cargo add rand

    let mut rnd = rand::rng();

    let random_num1 = rnd.random_range(-150..=250);

    println!("Random number 1: {random_num1}");

    let random_num2: f32 = rnd.random_range(-150.0..=250.0);

    println!("Random number 2: {random_num2}");
}

// Random number 1: 66
// Random number 2: -101.737785

// Random number 1: 94
// Random number 2: 97.77756
