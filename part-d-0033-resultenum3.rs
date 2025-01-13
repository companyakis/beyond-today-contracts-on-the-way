fn main() {

    let result1 = division(12 as f64, 4.25);

    let result2 = division(12 as f64, 0.0);

    println!("Result 1: {:?} and result 2: {:?}", result1, result2); // Result 1: Ok(2.823529411764706) and result 2: Err("Zero division error!")

    match result1 {
        Ok(r) => println!("Division result 1: {r}"), // Division result 1: 2.823529411764706
        Err(e) => println!("Error info: {e}")
    }

    match result2 {
        Ok(r) => println!("Division result 2: {r}"),
        Err(e) => println!("Error info: {e}") // Error info: Zero division error! 
    }

}

fn division(x: f64, y: f64) -> Result<f64, String> {

    if y == 0.0 {

        Err("Zero division error!".to_string())
    } 

    else {
        
        Ok(x /y)
    }
}




