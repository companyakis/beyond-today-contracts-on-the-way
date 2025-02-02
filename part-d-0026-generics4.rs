fn main() {

    let numbers1: [i16; 5] = [12, -54, 0, 21, 1000];

    let numbers2: [u8; 5] = [100, 12, 0, 65, 101];

    let numbers3: [f32; 5] = [-100.87, -12.0, -87.34, -1000.0, -97.54];

    println!("Max element of list 1: {:?}", find_max_element(&numbers1));

    println!("Max element of list 2: {:?}", find_max_element(&numbers2));

    println!("Max element of list 3: {:?}", find_max_element(&numbers3));

// Max element of list 1: 1000
// Max element of list 2: 101  
// Max element of list 3: -12.0

}

fn find_max_element<T: std::cmp::PartialOrd>(arr: &[T]) -> &T {

    let mut maximum = &arr[0];

    for a in arr.iter() {

        if a > maximum {

            maximum = a;
        }
    }

    maximum
}


