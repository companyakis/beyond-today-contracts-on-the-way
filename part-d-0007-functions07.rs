fn main() {

    let my_numbers = vec![21, -98, 321, 4214, 88, -9657];

    println!("My even numbers are: {:?}", find_even_numbers(my_numbers)) // My even numbers are: [-98, 4214, 88]

}


fn find_even_numbers(numbers: Vec<i32>) -> Vec<i32> {

    let mut evens: Vec<i32> = Vec::new();

    for num in numbers {

        if num % 2 == 0 {

            evens.push(num);
        }
    }

    evens
}
