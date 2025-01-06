// panic macro

fn main() {

    let year: u16 = 2025;

    println!("I hope {year} will be one of the best years!");

    panic!("Time elapses fast! Give a short break:)");

    /*
    
I hope 2025 will be one of the best years!
thread 'main' panicked at src/main.rs:7:5:
Time elapses fast! Give a short break:)
    
    */

    /* 
    panic!("Time elapses fast! Give a short break:)");
  |     ------------------------------------------------- any code following this expression is unreachable
8 |
9 | println!("Hi...")
  |     ^^^^^^^^^^^^^^^^^ unreachable expression
    */

}
