fn main() {

    let sales1: Result<u32, &str> = Result::Ok(55_000);

    //let sales2: Result<u32, &str> = Ok(155_000); // Result::Ok or Ok()

    let sales2: Result<u32, &str> = Result::Err("We don't have sales data!"); // Result::Err() or Err()

    println!("Sales 1: {sales1:?}");

    println!("Sales 2: {sales2:?}");
}

// Sales 1: Ok(55000)
// Sales 2: Err("We don't have sales data!")


