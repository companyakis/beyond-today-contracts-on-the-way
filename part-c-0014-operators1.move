module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        let x: u8 = 21;
        let y: u8 = 12;

        print(&(x + y));
        print(&(x - y));
        print(&(x * y));
        print(&(x / y));
        print(&(x % y));
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 33
// [debug] 9
// [debug] 252
// [debug] 1
// [debug] 9
