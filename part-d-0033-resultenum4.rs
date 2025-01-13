fn main() {

    let result1 = division(12 as f64, 4.25);

    let result2 = division(12 as f64, 0.0);

    println!("Result 1: {:?} and result 2: {:?}", result1, result2); // Result 1: Ok(2.823529411764706) and result 2: Err("Zero division error!")

    // unwrap, expect, unwrap_or, is_ok, is_err

    //println!("Result 2: {:?}", result2.unwrap()); 

    //println!("Result 2: {:?}", result2.expect("Careful! => "));

    //println!("Result 2: {:?}", result2.is_ok());

    println!("Result 2: {:?}", result2.is_err());

}

fn division(x: f64, y: f64) -> Result<f64, String> {

    if y == 0.0 {

        Err("Zero division error!".to_string())
    } 

    else {
        
        Ok(x /y)
    }
}




