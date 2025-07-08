fn main() {

    let num: u8 = 22;

    println!("{}", num + 1);

    let ref_num = &num;

    println!("{}", ref_num + 1); // Deref Coercions

    println!("{}", *ref_num + 1) // deref!

}
