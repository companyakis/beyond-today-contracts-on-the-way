fn main() {

    let sales = SalesData::Sales(67_000);

    let fixed_cost = SalesData::FixedCost(42_000);

    let variable_cost = SalesData::VariableCost(13_200);

    // shadowing and if let usage

    let sales = if let SalesData::Sales(value) = sales {value} else {0};

    let fixed_cost = if let SalesData::FixedCost(value) = fixed_cost {value} else {0};

    let variable_cost = if let SalesData::VariableCost(value) = variable_cost {value} else {0};

    let sales_result = profit_or_loss_calculator(sales, fixed_cost, variable_cost);

    println!("Profit or loss result: {}", sales_result) // Profit or loss result: 11800

}

#[derive(Debug)]
enum SalesData {

    Sales(u64),
    FixedCost(u64),
    VariableCost(u64)
}


fn profit_or_loss_calculator(s: u64, f: u64, v: u64) -> u64 {

        let result = s - (f + v);

        result
}


    
