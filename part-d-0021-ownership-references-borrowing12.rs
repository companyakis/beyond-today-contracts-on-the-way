fn main() {

    let mut num: u8 = 22;

    let ref_num = &mut num;

    //println!("{}", ref_num + 1); // Error!

    *ref_num = 100; // deref

    println!("{}", ref_num); // 100

    *ref_num = 11;

    println!("{}", ref_num); // 11

}
