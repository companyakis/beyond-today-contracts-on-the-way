fn main() {

    let c1: bool = 27 >= 12;

    let c2: bool = 12 != 12;

    let c3: bool = "İstanbul" == "istanbul";

    let result3 = c1 || (c2 && c3);

    println!("Result 3: {result3}");

    let result4 = (c1 || c2) && c3;

    println!("Result 4: {result4}");

}

// Result 3: true 
// Result 4: false

