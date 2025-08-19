module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        // "^(XOR): Compares the bits of two numbers. The result is 1 if the bits are different (0 and 1)."

        let x: u8 = 10; // 00001010
        let y: u8 = 10; // 00001010
        let z: u8 = 7; //  00000111

        print(&(x ^ y)); // 0 -> 00000000

        print(&(y ^ z)); // 13 -> 00001101

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
