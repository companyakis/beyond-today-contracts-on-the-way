module mustafa::learn_move {

    use std::debug::print;
   
    fun lets_move() {

     let num: u8 = 0; // "let mut" usage is not supported yet by APTOS cli!

     loop {

          print(&num);

          if (num >= 10) { break; };

          num += 1;
     }

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

