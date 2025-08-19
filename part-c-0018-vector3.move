module mustafa::learn_move {

    use std::debug::print;
    use std::vector;
  
    fun lets_move() {
   
        let years: vector<u16> = vector<u16>[2021, 2024, 2026, 2047, 1887];

        let look_year1: bool = vector::contains(&years, &2024);
        let look_year2: bool = vector::contains(&years, &1920);

        print(&look_year1);
        print(&look_year2);

        let first_year = vector::borrow(&years, 0);
        let third_year = vector::borrow(&years, 2);

        print(first_year);
        print(third_year);
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

//[debug] true
//[debug] false
//[debug] 2021
//[debug] 2026
