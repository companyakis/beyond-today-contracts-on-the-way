fn main() {

    // nested tuples => full name, birth year, inheritance rate

    let inheritors = (("Ayhan Bilir", 1965, 0.4), ("Kağan Bilir", 1968, 0.4), ("Aysel Sevdalı", 1987, 0.2));

    let rate1 = inheritors.0.2;
    let rate2 = inheritors.1.2;
    let rate3 = inheritors.2.2;

    println!("Inheritance rates: {rate1}, {rate2} and {rate3}.") // Inheritance rates: 0.4, 0.4 and 0.2.

}

