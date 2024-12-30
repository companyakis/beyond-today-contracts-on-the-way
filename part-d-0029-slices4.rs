fn main() {

let mut years:  [u16;7] = [2018, 2020, 2019, 2021, 2022, 2023, 2024];

let slice_years = &mut years[1..=5];

println!("Slice: {:?}", slice_years);

slice_years[4] = 1993;

println!("Slice: {:?}", slice_years);

println!("Years (updated): {:?}", years);

}

// Slice: [2020, 2019, 2021, 2022, 2023]
// Slice: [2020, 2019, 2021, 2022, 1993]
// Years (updated): [2018, 2020, 2019, 2021, 2022, 1993, 2024]

