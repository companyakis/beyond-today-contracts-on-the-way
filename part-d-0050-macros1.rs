macro_rules! cube {

    ($num: expr) => {
        
        $num * $num * $num
    };
}

fn main() {

    let cube1 = cube!(3);

    println!("Cube 1: {cube1}"); // Cube 1: 27

    let cube2 = cube!(2.25);

    println!("Cube 2: {cube2}"); // Cube 2: 11.390625

}



