fn main() {
    
    // stack data
    
    let year: u16 = 2025;
    
    // heap data
    
    let heap_year = Box::new(2025u16);
    
    println!("{}", year == *heap_year) // true
  
}
