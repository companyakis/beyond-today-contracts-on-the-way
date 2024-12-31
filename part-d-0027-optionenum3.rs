fn main() {

    // unwrap and expect

    let years = [1990, 2018, 2020, 2022, 2024];

    let names = ["Mustafa", "Aygün", "Bilge", "Kültigin", "Bumin", "Kutluk"];

    let year1 = years.get(3).unwrap();

    //let years2 = years.get(100).unwrap();

    println!("{:?}", year1); // 2022

    //println!("{:?}",years2); // thread 'main' panicked 

    let a_name = names.get(55).expect("Out of scope... Please, try a different index!");

    println!("{:?}", a_name); // Out of scope... Please, try a different index!

}

