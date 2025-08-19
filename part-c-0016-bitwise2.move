module mustafa::learn_move {

    use std::debug::print;
  
    fun lets_move() {

        // "| (OR): Compares the bits of two numbers. The result is 1 if there is a 1 in either bit."

        let x: u8 = 10; // 00001010
        let y: u8 = 10; // 00001010
        let z: u8 = 7; //  00000111

        print(&(x | y)); // 10 -> 00001010

        print(&(y | z)); // 15 -> 00001111

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
