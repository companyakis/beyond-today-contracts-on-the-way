fn main() {

    let family_ages: [u8; 5] = [66, 43, 35, 14, 9];

    let age_slice1 = &family_ages[0..=2];

    println!("{:?}", age_slice1);

    let age_slice2 = &family_ages[3..];

    println!("{:?}", age_slice2);

} 

// [66, 43, 35]
// [14, 9]
