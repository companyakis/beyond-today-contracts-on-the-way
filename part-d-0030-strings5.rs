fn main() {

    // cases

    let mut cities = " Ankara, İzmir, İstanbul, Adana, Kahramanmaraş     ";

    cities = cities.trim();

    println!("{}", cities.to_uppercase());

    println!("{}", cities.to_lowercase());

}

// ANKARA, İZMIR, İSTANBUL, ADANA, KAHRAMANMARAŞ
// ankara, i̇zmir, i̇stanbul, adana, kahramanmaraş
