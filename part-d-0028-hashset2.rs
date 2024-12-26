use std::collections::HashSet;

fn main() {

    let mut unique_customer_ids: HashSet<u16> = HashSet::new();

    unique_customer_ids.insert(1245);
    unique_customer_ids.insert(4296); // !
    unique_customer_ids.insert(3321);
    unique_customer_ids.insert(9874);
    unique_customer_ids.insert(12245);

    // contains

    println!("{}", unique_customer_ids.contains(&1111)); // false
    println!("{}", unique_customer_ids.contains(&4296)); // true

    // get

    println!("{:?}", unique_customer_ids.get(&1111)); // None   
    println!("{:?}", unique_customer_ids.get(&4296)); // Some(4296)
    println!("{:?}", unique_customer_ids.get(&4296).copied().unwrap()); // 4296   

    // remove 
    unique_customer_ids.remove(&4296);

    println!("IDs: {:?}", unique_customer_ids) // IDs: {1245, 12245, 3321, 9874}

}



