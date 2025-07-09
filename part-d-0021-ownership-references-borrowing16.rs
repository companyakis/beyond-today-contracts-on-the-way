fn main() {

    let mut name = String::from("Mustafa");

    let mut_ref_name = &mut name;

    mut_ref_name.push_str(" Büyükdereli");

    println!("{mut_ref_name}");

    println!("{name}");

    let ref_name = &name;

    println!("{ref_name}");

    let final_name = ref_name;

    println!("{final_name}");

    println!("{ref_name}");
}


