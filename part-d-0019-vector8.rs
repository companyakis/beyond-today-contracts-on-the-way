fn main() {

    let static_year_arr: [u16; 3] = [2020, 2021, 2022];

    // from array to vector

    let mut dynamic_year_vec = Vec::from(static_year_arr);

    dynamic_year_vec.push(2024);

}

