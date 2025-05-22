use rand::Rng;

fn main() {

    let mut rnd = rand::rng();

    let mut counter = 0;

    while counter < 50 {

        let her_age = rnd.random_range(1..50);

        if her_age % 35 == 0 {

            println!("We found her age {her_age} in {counter} try...");

            break
        }

        counter += 1;
    }


}

// We found her age 35 in 8 try...
// We found her age 35 in 22 try...   
