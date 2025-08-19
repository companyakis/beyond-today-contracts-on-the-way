module mustafa::learn_move {

    use std::debug::print;
    use std::vector;
  
    fun lets_move() {

        let _empty_vector: vector<u8> = vector::empty(); 
   
        let years: vector<u16> = vector<u16>[2021, 2024, 2026];

        print(&years); // [debug] [ 2021, 2024, 2026 ]

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
