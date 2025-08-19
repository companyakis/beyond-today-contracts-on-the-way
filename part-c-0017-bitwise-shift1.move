module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        // "Bitwise left shift moves all bits to the left, which effectively multiplies the number by a power of two."

        let x: u8 = 10; // 00001010
   
        print(&(x << 1)); // 20 -> 00010100

        print(&(x << 2)); // 40 -> 00101000

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
