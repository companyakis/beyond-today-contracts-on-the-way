fn main() {
    
    // stack data
    
    let stack_array: [u8; 5000] = [21; 5000];
    
    // heap data
    
    let heap_array = Box::new(stack_array);
    
    println!("{}", heap_array[5]) // 21
    
    
}
