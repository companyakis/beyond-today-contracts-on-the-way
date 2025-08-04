fn main() {

    let january_save = Financials::Save(4_600);

    let january_spend = Financials::Spend(3_400);

    println!("January spend and sale comparision 1 ==> {:?}",  january_save >= january_spend);

    println!("January spend and sale comparision 2 ==> {:?}",  january_save == january_spend);

    println!("January spend and sale comparision 3 ==> {:?}",  january_save < january_spend); 

}
#[derive(Debug, PartialEq, PartialOrd)]
enum Financials {
    Spend(u16),
    Save(u16)
}

// January spend and sale comparision 1 ==> true
// January spend and sale comparision 2 ==> false
// January spend and sale comparision 3 ==> false
