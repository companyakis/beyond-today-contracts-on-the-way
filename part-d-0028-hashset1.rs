use std::collections::HashSet;

fn main() {

    let mut unique_customer_ids: HashSet<u16> = HashSet::new();

    unique_customer_ids.insert(1245);
    unique_customer_ids.insert(4296); // !
    unique_customer_ids.insert(3321);
    unique_customer_ids.insert(9874);
    unique_customer_ids.insert(12245);

    println!("{:?}", unique_customer_ids); 

    unique_customer_ids.insert(4296); // !

    println!("{:?}", unique_customer_ids); 

}

// {4296, 12245, 1245, 3321, 9874}
// {4296, 12245, 1245, 3321, 9874}


