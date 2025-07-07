fn main() {

    let proverb  = "A rolling stone gathers no moss!".to_string();

    drop(proverb);

    // println!("My proverb: {proverb}") // value borrowed here after move

}
