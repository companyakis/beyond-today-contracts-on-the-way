fn main() {

    // trims

    let mut cities = " Ankara, İzmir, İstanbul, Adana, Kahramanmaraş     ";

    println!("{}", cities.trim());

    println!("{}", cities.trim_start());

    println!("{}", cities.trim_end());

    // reassign

    cities = cities.trim();

    println!("{cities}")

}


