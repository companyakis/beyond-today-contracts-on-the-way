module mustafa::learn_move {

    use std::debug::print;
   
    fun lets_move() {

       let x: u8 = 12;
       let y: u8 = 7;
       let z: u8 = 10;

       let result: u8 = if ((x > y) && (y == z)) {

            5

       } else if ((x != y) || (z > x)) {

            3

       } else {

            1
       };

       print(&result); // 3

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
