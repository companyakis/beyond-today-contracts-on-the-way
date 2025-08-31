fn main() {

    let d1 = match division(12.23, 3.0) {
        
        Ok(v) => v,
        Err(e) => panic!("Error: {e}")
    };
    
    println!("Division result: {d1}");
    
    let new_variable = d1 + 3 as f32;
    
    println!("New variable: {new_variable}");

}


fn division(a: f32, b: f32) -> Result<f32, String> {
    
    if b != 0.0 {
        
        Ok(a / b)
        
    } else {
    
        Err("Zero division error!".to_string())
    }
}

// Division result: 4.0766664
// New variable: 7.0766664
