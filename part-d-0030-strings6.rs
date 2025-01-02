fn main() {

    // replace

    let mut cities = " Ankara, İzmir, İstanbul, Adana, Kahramanmaraş     ";

    cities = cities.trim();

    println!("{}", cities.replace(", ", "---")) // Ankara---İzmir---İstanbul---Adana---Kahramanmaraş

}


