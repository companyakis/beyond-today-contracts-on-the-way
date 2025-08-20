module mustafa::learn_move {

    use std::debug::print;
   
    fun lets_move() {

     let num: u8 = 0; // "let mut" usage is not supported yet by APTOS cli!

     while (num <= 50) {

          num += 5;

          if (num == 30) { continue; };

          if (num == 40) { break; };

          print(&num);
     }

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

//[debug] 5
//[debug] 10
//[debug] 15
//[debug] 20
//[debug] 25
//[debug] 35
