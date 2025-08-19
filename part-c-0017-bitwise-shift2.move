module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        // "Bitwise right shift moves all bits to the right, which effectively performs an integer division by a power of two."

        let x: u8 = 10; // 00001010
   

        print(&(x >> 1)); // 5 -> 00000101

        print(&(x >> 2)); // 2 -> 00000010

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
