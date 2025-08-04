fn main() {

    let january_save = Financials::Save(4_600);

    let january_spend = Financials::Spend(3_400);

    println!("January spend and sale: {:?} - {:?}", january_save, january_spend) // January spend and sale: Save(4600) - Spend(3400)

}
#[derive(Debug)]
enum Financials {
    Spend(u16),
    Save(u16)
}
