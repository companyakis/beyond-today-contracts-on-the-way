module mustafa::learn_move {

    use std::debug::print;
   
    fun lets_move() {

     // "let mut" usage is not supported yet by APTOS cli!

     let num: u8 = 0; 

     let upper_number: u8 = 15;

     for (num in 7..upper_number) {

          print(&num);
     }

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 7
// [debug] 8
// [debug] 9
// [debug] 10
// [debug] 11
// [debug] 12
// [debug] 13
// [debug] 14
