fn main() {

let years:  [u16;7] = [2018, 2020, 2019, 2021, 2022, 2023, 2024];

let _s1 = &years[2..];

let _s2 = &years[1..=4];

let _s3 = &years[..4];

let _s4 = &years[2..5];

let _s5 = &years[2];

}



