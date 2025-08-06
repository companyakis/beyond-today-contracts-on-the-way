fn main() {

    let season1 = season_converter(5);

    println!("Season 1 is {season1}");

    let season2 = season_converter(27);

    println!("Season 2 is {season2}");
}

fn season_converter(month: u8) -> &'static str {

    match month {
        12 | 1 | 2 => "Winter",
        3..=5 => "Spring",
        6..=8 => "Summer",
        9..=11 => "Autumn",
        _ => "Unknown"
    }
}


// Season 1 is Spring
// Season 2 is Unknown
