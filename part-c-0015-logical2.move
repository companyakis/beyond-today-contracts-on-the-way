module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        let x: bool = true;
        let y: bool = false;

        print(&((x == y) && ( x != y)));
        print(&((x != y) && (y != x)));
        print(&((x != y) || (y == x)));
        print(&((x == y) || (y != x)));
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

//[debug] false
//[debug] true
//[debug] true
//[debug] true
