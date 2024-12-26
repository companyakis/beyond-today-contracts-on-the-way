use std::collections::HashSet;

fn main() {

    let mut class_a: HashSet<&str> = HashSet::new();
    let mut class_b: HashSet<&str> = HashSet::new();

    class_a.insert("Mustafa");
    class_a.insert("Aygün");
    class_a.insert("Kutluk");

    class_b.insert("Bilge");
    class_b.insert("Bumin");
    class_b.insert("Mustafa");
    class_b.insert("Kültigin");

    // union, difference, symmetric difference, disjoint, subset, superset

    println!("{:?}", class_a.union(&class_b));

    println!("{:?}", class_a.difference(&class_b));

    println!("{:?}", class_a.symmetric_difference(&class_b));

    println!("{:?}", class_a.is_disjoint(&class_b));

    println!("{:?}", class_a.is_subset(&class_b));

    println!("{:?}", class_a.is_superset(&class_b));
}

// ["Mustafa", "Bilge", "Bumin", "Kültigin", "Kutluk", "Aygün"]
// ["Kutluk", "Aygün"]
// ["Kutluk", "Aygün", "Bilge", "Bumin", "Kültigin"]
// false
// false
// false

