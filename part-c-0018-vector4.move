module mustafa::learn_move {

    use std::debug::print;
    use std::vector;
  
    fun lets_move() {
   
        let years: vector<u16> = vector<u16>[2021, 2024, 2026, 2047, 1887];

        vector::swap(&mut years, 0, 3);

        print(&years); // [debug] [ 2047, 2024, 2026, 2021, 1887 ]

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
