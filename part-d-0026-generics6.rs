fn main() {

    let year_data1: YearData<&str> =  YearData {month: "January", currentYear: "2025".to_string()};

    let year_data2: YearData<u8> = YearData {month: 1, currentYear: String::from("2025")};

}

struct YearData<T> {
  
    month: T,
    currentYear: String
}





