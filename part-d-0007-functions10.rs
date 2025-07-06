fn main() {

    // function in function example

    let result1 = sum_or_diff_or_mult('m', -11, 22);

    println!("Result 1: {result1}"); // Result 1: -242

    let result2 = sum_or_diff_or_mult('d', -11, 22);

    println!("Result 2: {result2}"); // Result 2: -33

    let result3 = sum_or_diff_or_mult('s', -11, 22);

    println!("Result 3: {result3}"); // Result 3: 11 

    let result4 = sum_or_diff_or_mult('x', -11, 22);

    println!("Result 4: {result4}") // Result 4: 0  
}


fn sum(x: i64, y: i64) -> i64 {

    x + y
}

fn diff(x: i64, y: i64) -> i64 {

    x - y
}

fn sum_or_diff_or_mult(signer: char, a: i64, b: i64) -> i64 {

    if signer == 'm' {
        a * b
    }

    else if signer == 's' {
        sum(a, b)
    }

    else if signer == 'd' {
        diff(a, b)
    }

    else {
        0
    }
}
