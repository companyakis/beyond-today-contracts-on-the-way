fn main() {

    let ages: [u8; 6]= [22, 34, 56, 12, 76, 65];

    for age in &ages{

        print!("{age} | ") // 22 | 34 | 56 | 12 | 76 | 65 |
    
    }

}
