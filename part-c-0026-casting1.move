module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let age: u8 = 10;
      
        let year: u16 = 2025;
      
        //let birth = year - age; // error: cannot use `u8` with an operator which expects a value of type `u16`
        // print(&birth);

        let birth_year = year - (age as u16);

        print(&birth_year); // [debug] 2015
   
    }

    #[test]
    fun testing() {

        lets_move();
    }

}
