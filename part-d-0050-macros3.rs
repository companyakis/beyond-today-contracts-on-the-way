macro_rules! sum_as 

 {
    ($x: expr, $y: expr, $z: expr, $d_type: ty) => {
        
        $x as $d_type + $y as $d_type + $z as $d_type
    };
}

fn main() {

    let sum1 = sum_as!(45, 72, 97, u8);

    println!("sum => u8: {sum1}");

    let sum2 = sum_as!(45, 72, 97.12, f32);

    println!("sum => f32: {sum2}");

}

// sum => u8: 214
// sum => f32: 214.12

