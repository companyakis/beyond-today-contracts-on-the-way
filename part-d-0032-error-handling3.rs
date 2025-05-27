fn main() {

    let division1 = calculate_division(125.456, 0.0);

    match division1 {

        Ok(r) => println!("Division 1 result: {r}"),

        Err(e) => println!("{e}") // Zero division error! Please, be careful!
        
    }

    let division2 = calculate_division(1254.9887, -111.566);

    match division2 {

        Ok(r) => println!("Division 2 result: {r}"), // Division 2 result: -11.248845526414858

        Err(e) => println!("{e}")
        
    }

}

fn calculate_division(x: f64, y: f64) -> Result<f64, String> {

    if y == 0_f64 {

        Err("Zero division error! Please, be careful!".to_string())
    } 

    else {

        Ok(x / y)
    }
}

