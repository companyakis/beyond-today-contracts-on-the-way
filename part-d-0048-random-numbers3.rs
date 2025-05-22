use rand::Rng;

fn main() {

    let ages1 = create_random_ages(80, 10);

    println!("Age vector 1: {:?}", ages1); 

    let ages2 = create_random_ages(100, 12);

    println!("Age vector 2: {:?}", ages2);

// Age vector 1: [42, 6, 37, 49, 46, 33, 15, 41, 67, 34]
// Age vector 2: [86, 95, 96, 19, 69, 25, 14, 20, 26, 12, 45, 11]


}


fn create_random_ages(max_age: u8, vec_limit: u8) -> Vec<u8> {

    let mut rnd = rand::rng();

    let mut ages: Vec<u8> = Vec::new();

    for i in 0..vec_limit {

        let rand_age = rnd.random_range(0..=max_age);

        if ages.contains(&rand_age) {
            continue;
        }

        ages.push(rand_age);
    }

    ages
}

