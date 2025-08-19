module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        let x: u8 = 21;
        let y: u8 = 12;

        print(&(x == y));
        print(&(x != y));
        print(&(x >= y));
        print(&(x < y));
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

//[debug] false
//[debug] true
//[debug] true
//[debug] false
