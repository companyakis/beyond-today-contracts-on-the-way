module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        // "& (AND): Compares the bits of two numbers. The result is 1 only if both bits are 1."

        let x: u8 = 10; // 00001010
        let y: u8 = 10; // 00001010
        let z: u8 = 7; //  00000111

        print(&(x & y)); // 10 -> 00001010

        print(&(y & z)); // 2 -> 00000010 

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
