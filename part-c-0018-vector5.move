module mustafa::learn_move {

    use std::debug::print;
    use std::vector;
  
    fun lets_move() {
   
        let years: vector<u16> = vector<u16>[2021, 2024, 2026, 2047, 1887];

        let new_years: vector<u16> = vector<u16>[2050, 2060];

        vector::append(&mut years, new_years); // [debug] [ 2021, 2024, 2026, 2047, 1887, 2050, 2060 ]

        print(&years);

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
