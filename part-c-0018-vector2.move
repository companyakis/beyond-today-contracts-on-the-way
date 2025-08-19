module mustafa::learn_move {

    use std::debug::print;
    use std::vector;
  
    fun lets_move() {
   
        let years: vector<u16> = vector<u16>[2021, 2024, 2026];

        vector::push_back(&mut years, 1990);

        print(&years); // [debug] [ 2021, 2024, 2026, 1990 ]

        vector::pop_back(&mut years);

        print(&years); // [debug] [ 2021, 2024, 2026 ]

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
